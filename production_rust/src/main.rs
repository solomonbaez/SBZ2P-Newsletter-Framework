use production_rust::configuration::get_configuration;
use production_rust::startup::Application;
use production_rust::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("production_rust".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");

    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
