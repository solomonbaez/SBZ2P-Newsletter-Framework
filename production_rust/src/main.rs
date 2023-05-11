use production_rust::configuration::get_configuration;
use production_rust::email_client::EmailClient;
use production_rust::startup::run;
use production_rust::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("production_rust".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect_lazy_with(config.database.with_db());

    let sender_email = config
        .email_client
        .sender()
        .expect("Invalid email address.");
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.auth_token,
    );

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    println!(
        "\n{}\n",
        format_args!(
            "Running Server -- http://127.0.0.1:{}",
            listener.local_addr().unwrap().port()
        )
    );

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
