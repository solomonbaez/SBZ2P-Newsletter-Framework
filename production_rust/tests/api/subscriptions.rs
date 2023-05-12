use crate::helpers::spawn_app;

#[tokio::test] // valid form data returns 200
async fn subscribe_returns_200() {
    let app = spawn_app().await; // Future

    let client = reqwest::Client::new();

    let body = "name=Aeonid%20Thiel&email=calth_invigilatus%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

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
async fn subscribe_returns_400_with_empty_fields() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=&email=raptor_imperialis%40@gmail.com", "empty name"),
        ("name=Aeonid&email=", "empty email"),
        ("name=Aeonid&email=invalid-email", "invalid email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return with 400: {}.",
            error_message,
        );
    }
}

#[tokio::test] // Parametrized Test: missing form data returns 400
async fn subscribe_returns_400() {
    let app = spawn_app().await; // Future
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Aeonid%20Thiel", "Missing the email."),
        ("email=calth_invigilas%40gmail.com", "Missing the name."),
        ("", "Missing both fields."),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400: {}",
            error_message
        );
    }
}
