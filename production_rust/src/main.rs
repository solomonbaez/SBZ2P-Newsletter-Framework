use production_rust::configuration::get_configuration;
use production_rust::startup::run;
use production_rust::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("production_rust".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    println!(
        "\n{}\n",
        format_args!(
            "Running Server -- http://127.0.0.1:{}",
            listener.local_addr().unwrap().port()
        )
    );

    run(listener, connection_pool)?.await
}
