use devapi::configuration;
use devapi::startup::Application;
use devapi::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("devapi".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
