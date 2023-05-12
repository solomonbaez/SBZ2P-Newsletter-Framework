use once_cell::sync::Lazy;
use production_rust::configuration::{get_configuration, DatabaseSettings};
use production_rust::email_client::EmailClient;
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

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to read configuration.");
    config.database.database_name = Uuid::new_v4().to_string();
    // println!("{}", config.database.database_name);

    let connection_pool = test_database(&config.database).await;
    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid email address.");

    let timeout = config.email_client.timeout();
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.auth_token,
        timeout,
    );

    let server = production_rust::startup::run(listener, connection_pool.clone(), email_client)
        .expect("Failed to bind address.");

    let _ = tokio::spawn(server);

    TestApp {
        address: address,
        pg_pool: connection_pool,
    }
}

async fn test_database(config: &DatabaseSettings) -> PgPool {
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
