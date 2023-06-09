use production_rust::configuration::get_configuration;
use production_rust::issue_delivery_worker::run_worker_until_stopped;
use production_rust::startup::Application;
use production_rust::telemetry::{get_subscriber, init_subscriber};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("production_rust".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");
    let application = Application::build(config.clone()).await?;

    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(config));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}
