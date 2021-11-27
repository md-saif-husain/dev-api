use devapi::configuration;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to database");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    devapi::startup::run(listener, connection_pool)?.await
}
