use devapi::configuration;
use devapi::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("devapi".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();
    let connection_pool = PgPool::connect_lazy(&connection_string)
        .expect("Failed to connect to database");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    devapi::startup::run(listener, connection_pool)?.await
}
