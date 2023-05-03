#[tokio::test]
async fn health_check_confirm() {
    spawn_app();

    let client = reqwest::Client::new();

    // check that client is exposed at /health_check
    // + is behind GET + always returns a 200
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = production_rust::run().expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}
