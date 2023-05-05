use production_rust::configuration::get_configuration;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = production_rust::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address.");

    let _ = tokio::spawn(server);

    TestApp {
        address: address,
        pg_pool: connection_pool,
    }
}

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

#[tokio::test] // Parametrized Test: missing form data returns 400
async fn subscribe_returns_400() {
    let app = spawn_app().await; // Future
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Aeonid%20Thiel", "Missing the email."),
        ("email=calth_invigilas%40gmail.com", "Missing the name."),
        ("", "Missing both fields."),
    ];

    for (_invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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
