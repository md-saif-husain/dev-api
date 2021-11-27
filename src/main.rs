use devapi::configuration;
use devapi::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener)?.await
}
