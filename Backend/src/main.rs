#![allow(unused)]
mod config;
mod start_server;
use crate::config::Config;
use start_server::start;
use tracing::{info, debug};

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    debug!("main fn called");
    let config = Config::from_env().expect("failed to get the Config");
    start(config).await?;
    Ok(())
}
