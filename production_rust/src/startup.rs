use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub async fn build(config: Settings) -> Result<Server, std::io::Error> {
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect_lazy_with(config.database.with_db());

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

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)?;
    println!(
        "\n{}\n",
        format_args!(
            "Running Server -- http://127.0.0.1:{}",
            listener.local_addr().unwrap().port()
        )
    );

    run(listener, connection_pool, email_client)
}

pub fn run(
    listener: TcpListener,
    pg_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(pg_pool);
    let email_client = web::Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
