extern crate lazy_static;

mod start_server;
use lib::{get_subscriber, init_subscriber, Settings};
use start_server::start;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // set logger
    let config = Settings::get_config().expect("failed to get the Application Settings");
    let subscriber = get_subscriber(&config.application, std::io::stdout);
    init_subscriber(subscriber);
    start(config).await?;
    Ok(())
}
