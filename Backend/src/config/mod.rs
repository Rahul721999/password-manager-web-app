#![allow(unused)]
use anyhow::Context;
use dotenv::dotenv;
use sqlx::{PgPool};
use std::env;
use tracing::debug;
// use tracing_subscriber::{self, EnvFilter};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub db_url: String,
    pub jwt_key: String,
    pub jwt_exp: usize,
}
impl Config {
    /// this fn is to load configuration from env
    pub fn from_env() -> anyhow::Result<Config> {
        // loadinf env variables
        dotenv().ok();

        debug!("Loading env Configuration");

        // get the port id from .env
        let port = env::var("PORT").expect("failed to get the PORT from env");
        // parse string_port to i32
        let port = port.parse::<i32>().context("failed to parse PORT")?;
        let exp = env::var("JWT_EXP")
            .expect("JWT_EXP is not set")
            .parse::<usize>()
            .expect("failed to parse JWT_EXP");
        //return the config
        Ok(Config {
            host: env::var("HOST").context("HOST is not set")?,
            port,
            db_url: env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
            jwt_key: env::var("JWT_KEY").context("JWT_KEY is not set")?,
            jwt_exp: exp,
        })
    }
}

/// fn to Connect the db...
pub fn run(database_url: &str) -> PgPool {
    match PgPool::connect_lazy(database_url)
    {
        Ok(pool) => {
            debug!("✅ Connecting to PSQL db Successfully");
            pool
        }
        Err(_err) => {
            panic!("🔥 failed to connect PSQL_DB");
        }
    }
}
