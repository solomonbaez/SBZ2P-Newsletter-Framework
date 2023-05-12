use production_rust::configuration::get_configuration;
use production_rust::startup::build;
use production_rust::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("production_rust".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");

    let server = build(config).await?;
    server.await?;
    Ok(())
}
