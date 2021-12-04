use devapi::configuration;
use devapi::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("devapi".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    devapi::startup::run(listener, connection_pool)?.await
}
