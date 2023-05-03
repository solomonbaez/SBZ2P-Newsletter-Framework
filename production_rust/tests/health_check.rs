use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let server = production_rust::startup::run(listener).expect("Failed to bind address.");

    let _ = tokio::spawn(server);
    // inform the caller of the application address
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test] // health check endpoint is valid
async fn health_check_confirm() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test] // valid form data returns 200
async fn subscribe_returns_200() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=Roubute%20Guilliman&email=roubute_guilliman%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test] // Parametrized Test: missing form data returns 400
async fn subscribe_returns_400() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Roubute%20Guilliman", "Missing the email."),
        ("email=roubute_guilliman%40gmail.com", "Missing the name."),
        ("", "Missing both fields."),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
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
