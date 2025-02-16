use crate::AppError;
use config::{Config, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode},
    ConnectOptions,
};

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub name: String,
    pub require_ssl: bool,
}
///implementing database settings
impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_req = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_req)
    }
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub jwt_key: Secret<String>,
    pub jwt_exp: u16,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrontendSettings {
    pub url: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RedisDBSettings {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub frontend: FrontendSettings,
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub redis: RedisDBSettings,
}
impl Settings {
    pub fn get_config() -> Result<Settings, AppError> {
        // Get the base configuration directory relative to the current working directory
        let base_path = std::env::current_dir().expect("Failed to get current directory");
        let config_dir = base_path.join("configuration");

        // Load the base configuration file (contains default values)
        let config =
            Config::builder().add_source(File::from(config_dir.join("base")).required(true));

        // Determine runtime environment (defaults to "local" if not set)
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Invalid APP_ENVIRONMENT value");

        // Load environment-specific configuration to override base values
        let config = config.add_source(File::from(config_dir.join(environment.as_str())));

        // Load environment variables prefixed with "APP__" (e.g., APP__DATABASE__HOST → settings.database.host)
        let set_config = match config
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
        {
            Ok(config) => config,
            Err(err) => {
                tracing::error!("❌ Failed to create configuration: {}", err);
                return Err(AppError::InternalServerError(
                    "Configuration Error".to_string(),
                ));
            }
        };

        // Deserialize configuration into a strongly-typed Settings struct
        let settings = set_config
            .try_deserialize::<Settings>()
            .expect("Failed to parse configuration");
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
        tracing::info!("🚀 Database url: {}", database_url);
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(self.database.with_db())
    }
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Docker,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Docker => "docker",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "docker" => Ok(Self::Docker),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported env. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}
