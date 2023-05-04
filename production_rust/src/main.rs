use production_rust::configuration::get_configuration;
use production_rust::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    println!(
        "\n{}",
        format_args!(
            "Running Server -- http://127.0.0.1:{}",
            listener.local_addr().unwrap().port()
        )
    );

    run(listener)?.await
}
