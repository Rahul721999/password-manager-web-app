extern crate lazy_static;

mod config;
mod start_server;
use lib::config::Config;
use lib::{get_subscriber, init_subscriber};
use start_server::start;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    // set logger
    let subscriber = get_subscriber(std::io::stdout);
    init_subscriber(subscriber);
    let config = Config::from_env().expect("failed to get the Config");
    start(config).await?;
    Ok(())
}
