use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

// will be expanded to include flash message confirmation
#[tokio::test]
async fn subscribe_endpoint_is_accessable() {
    let app = spawn_app().await;

    let html_page = app.get_subscribe_html().await;
    assert!(html_page.contains("<h2>Subscribe</h2>"));
}

#[tokio::test] // valid form data returns 200
async fn valid_subscribe_returns_200() {
    let app = spawn_app().await; // Future

    let body = "name=Aeonid%20Thiel&email=calth_invigilatus%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscribers(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn valid_subscriber_persists() {
    let app = spawn_app().await;

    let body = "name=Aeonid%20Thiel&email=calth_invigilatus%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscribers(body.into()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.pg_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "calth_invigilatus@gmail.com");
    assert_eq!(saved.name, "Aeonid Thiel");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn invalid_subscribe_returns_400_empty_fields() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=&email=raptor_imperialis%40@gmail.com", "empty name"),
        ("name=Aeonid&email=", "empty email"),
        ("name=Aeonid&email=invalid-email", "invalid email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscribers(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return with 400: {}.",
            error_message,
        );
    }
}

#[tokio::test] // Parametrized Test: missing form data returns 400
async fn invald_subscribe_returns_400_missing_data() {
    let app = spawn_app().await; // Future

    let test_cases = vec![
        ("name=Aeonid%20Thiel", "Missing the email."),
        ("email=calth_invigilata%40gmail.com", "Missing the name."),
        ("", "Missing both fields."),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscribers(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400: {}",
            error_message
        );
    }
}

#[tokio::test]
async fn valid_subscribe_sends_confirmation_email() {
    let app = spawn_app().await;

    let body = "name=Aeonid%20Thiel&&email=calth_invigilata%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscribers(body.into()).await;
}

#[tokio::test]
async fn valid_subscribe_sends_confirmation_email_link() {
    let app = spawn_app().await;

    let body = "name=Aeonid%20Thiel&&email=calth_invigilata%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscribers(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let confirmation_links = app.get_confirmation_links(email_request);

    assert_eq!(confirmation_links.html_link, confirmation_links.text_link)
}

#[tokio::test]
async fn subscribe_fails_with_fatal_database_error() {
    let app = spawn_app().await;

    let body = "name=Aeonid%20Thiel&&email=calth_invigilata%40gmail.com";

    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN email;",)
        .execute(&app.pg_pool)
        .await
        .unwrap();

    let response = app.post_subscribers(body.into()).await;

    assert_eq!(response.status().as_u16(), 500);
}
