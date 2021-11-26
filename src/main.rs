use devapi::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("Failed to bind random port");
    run(listener)?.await
}
