#![allow(unused)]
use anyhow::Context;
use dotenv::dotenv;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tracing::debug;
use crate::AppError;
use config::{Config, File};

#[derive(Debug, Deserialize,Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub jwt_key: Secret<String>,
    pub jwt_exp: u16,
    pub log_level: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

impl Settings {
    pub fn get_config() -> Result<Settings, AppError> {
        let base_path = std::env::current_dir().expect("Failed to get the curr dir");
        let config_dir = base_path.join("configuration");
        let config =
            Config::builder().add_source(File::from(config_dir.join("base")).required(true));
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse App Environment");
        // let environment = Environment::Production;
        let settging_config = match config
            .add_source(File::from(config_dir.join(environment.as_str())))
            .build()
        {
            Ok(config) => config,   
            Err(err) => {
                tracing::error!("❌Failed to create configuration: {}", err);
                return Err(AppError::InternalServerError(
                    "Application Configuration Error".to_string(),
                ));
            }
        };
        let settings = settging_config
            .try_deserialize::<Settings>()
            .expect("Failed to parse config to Settings Struct");
        Ok(settings)
    }

    /// fn to Connect the db...
    pub fn run(&self) -> PgPool {
        let db = &self.database;
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db.username,
            db.password.expose_secret(),
            db.host,
            db.port,
            db.name
        );
        match PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy(&database_url)
        {
            Ok(pool) => {
                debug!("✅ Connecting to PSQL db Successfully");
                pool
            }
            Err(err) => {
                panic!("🔥 failed to connect PSQL_DB: {}", err);
            }
        }
    }
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported env. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
