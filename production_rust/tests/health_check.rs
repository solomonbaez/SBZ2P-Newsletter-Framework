#[tokio::test]
async fn health_check_confirm() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    // check that client is exposed at /health_check
    // + is behind GET + always returns a 200
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let server = production_rust::run(listener).expect("Failed to bind address.");

    let _ = tokio::spawn(server);
    // inform the caller of the application address
    format!("http://127.0.0.1:{}", port)
}
