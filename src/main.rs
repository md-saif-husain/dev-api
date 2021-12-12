use devapi::configuration;
use devapi::email_client::EmailClient;
use devapi::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("devapi".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(15))
        .connect_lazy_with(config.database.with_db());
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let sender_email = config
        .email_client
        .sender()
        .expect("Failed to get sender email");
    let email_client = EmailClient::new(
        config.email_client.base_url,
        sender_email,
        config.email_client.authorization_token,
    );
    devapi::startup::run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
