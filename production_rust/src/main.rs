use production_rust::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    println!(
        "\n{}",
        format!("Running Server -- http://127.0.0.1:{}", port)
    );

    run(listener)?.await
}
