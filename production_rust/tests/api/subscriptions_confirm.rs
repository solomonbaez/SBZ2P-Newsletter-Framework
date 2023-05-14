use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn reject_tokenless_confirmations_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn valid_link_200() {
    let app = spawn_app().await;

    let body = "name=Aeonid%20Thiel&email=calth_invigilatus%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscribers(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_link = app.get_confirmation_links(&email_request);

    let response = reqwest::get(confirmation_link.html_link).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn valid_link_confirms_subscriber() {
    let app = spawn_app().await;
    let body = "name=Aeonid%20Thiel&email=calth_invigilatus%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscribers(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_link = app.get_confirmation_links(&email_request);

    reqwest::get(confirmation_link.html_link)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.pg_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "calth_invigilatus@gmail.com");
    assert_eq!(saved.name, "Aeonid Thiel");
    assert_eq!(saved.status, "confirmed");
}
