use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

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

    println!("Testing...");
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.pg_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "calth_invigilatus@gmail.com");
    assert_eq!(saved.name, "Aeonid Thiel")
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

    let email_body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let html_link = get_link(&email_body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(&email_body["TextBody"].as_str().unwrap());

    assert_eq!(html_link, text_link)
}
