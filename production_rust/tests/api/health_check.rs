use crate::helpers::spawn_app;

#[tokio::test] // health check endpoint is valid
async fn health_check_confirm() {
    let app = spawn_app().await; // Future
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
