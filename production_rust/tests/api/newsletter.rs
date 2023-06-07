use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::Fake;
use std::time::Duration;
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};
// use wiremock::MockBuilder;

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();
    let body = serde_urlencoded::to_string(&serde_json::json!({
        "name": name,
        "email": email,
    }))
    .unwrap();

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
    app.post_subscribers(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_link.html_link)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn newsletters_unavailable_for_unconfirmed_subscribers() {
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as text",
        "html_content": "<p>Newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published.</i></p>"));
}

#[tokio::test]
async fn newsletters_available_for_confirmed_subscribers() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as text",
        "html_content": "<p>Newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published.</i></p>"));
}

#[tokio::test]
async fn login_required_to_see_newsletter_form() {
    let app = spawn_app().await;

    let response = app.get_publish_newsletter().await;
    assert_is_redirect_to(&response, "/login")
}

#[tokio::test] // check this test at some point, should have failed w/o idpkey
async fn login_required_to_publish_newsletter() {
    let app = spawn_app().await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as text",
        "html_content": "<p>Newsletter body as html</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain text",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published.</i></p>"));

    let response = app.post_publish_newsletter(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_publish_newsletter_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published.</i></p>"));
}

#[tokio::test]
async fn graceful_handling_concurrent_form_submission() {
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(2)))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "text_content": "Newsletter body as plain test",
        "html_content": "<p>Newsletter body as HTML</p>",
        "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response_1 = app.post_publish_newsletter(&newsletter_request_body);
    let response_2 = app.post_publish_newsletter(&newsletter_request_body);
    let (response_1, response_2) = tokio::join!(response_1, response_2);

    assert_eq!(response_1.status(), response_2.status());
    assert_eq!(
        response_1.text().await.unwrap(),
        response_2.text().await.unwrap()
    );
}

// fn when_sending_an_email() -> MockBuilder {
//     Mock::given(path("/email")).and(method("POST"))
// }

// #[tokio::test]
// async fn transient_errors_do_not_cause_duplicate_deliveries_on_retries() {
//     let app = spawn_app().await;
//     let newsletter_request_body = serde_json::json!({
//         "title": "Newsletter title",
//         "text_content": "Newsletter body as plain text",
//         "html_content": "<p>Newsletter body as HTML<p>",
//         "idempotency_key": uuid::Uuid::new_v4().to_string()
//     });

//     create_confirmed_subscriber(&app).await;
//     create_confirmed_subscriber(&app).await;
//     app.test_user.login(&app).await;

//     when_sending_an_email()
//         .respond_with(ResponseTemplate::new(200))
//         .up_to_n_times(1)
//         .expect(1)
//         .mount(&app.email_server)
//         .await;
//     when_sending_an_email()
//         .respond_with(ResponseTemplate::new(500))
//         .up_to_n_times(1)
//         .expect(1)
//         .mount(&app.email_server)
//         .await;

//     let response = app.post_publish_newsletter(&newsletter_request_body).await;
//     assert_eq!(response.status().as_u16(), 500);

//     when_sending_an_email()
//         .respond_with(ResponseTemplate::new(200))
//         .expect(1)
//         .named("Delivery retry")
//         .mount(&app.email_server)
//         .await;

//     let response = app.post_publish_newsletter(&newsletter_request_body).await;
//     assert_eq!(response.status().as_u16(), 303);
// }
