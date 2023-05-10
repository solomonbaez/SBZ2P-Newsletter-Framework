use once_cell::sync::Lazy;
use production_rust::configuration::{get_configuration, DatabaseSettings};
use production_rust::telemetry::{get_subscriber, init_subscriber};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to read configuration.");
    config.database.database_name = Uuid::new_v4().to_string();
    // println!("{}", config.database.database_name);

    let connection_pool = test_database(&config.database).await;

    let server = production_rust::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address.");

    let _ = tokio::spawn(server);

    TestApp {
        address: address,
        pg_pool: connection_pool,
    }
}

pub async fn test_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, &config.database_name).as_str())
        .await
        .expect("Failed to create test database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("../migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
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
