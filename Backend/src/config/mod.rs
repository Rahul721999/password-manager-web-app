// #![allow(unused)]
use anyhow::Context;
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tracing::{debug,instrument};
// use tracing_subscriber::{self, EnvFilter};
#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub db_url: String,
}

impl Config {
    /// this fn is to load configuration from env
    #[instrument]
    pub fn from_env() -> anyhow::Result<Config> {
        // loadinf env variables
        dotenv().ok();

        debug!("Loading env Configuration");

        // get the port id from .env
        let port = env::var("PORT").expect("failed to get the PORT from env");
        // parse string_port to i32
        let port = port.parse::<i32>().context("failed to parse PORT")?;

        //return the config
        Ok(Config {
            host: env::var("HOST").context("HOST is not set")?,
            port,
            db_url: env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
        })
    }

    #[instrument]
    pub async fn run(database_url : String) -> PgPool {
        debug!("Connecting to PSQL db");
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await{
                Ok(pool) => return pool,
                Err(_err) =>{panic!("failed to connect PSQL_DB");}
            }
    }
}
