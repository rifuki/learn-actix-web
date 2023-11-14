use config::{Config, ConfigError, File};
use serde::Deserialize;
use sqlx::{self, postgres::PgConnectOptions};
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub test_client: TestClientSettings
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestClientSettings {
    pub base_url: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

impl DatabaseSettings {
    pub fn parse_connection_string(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
    }

    pub fn database_connection_string(&self) -> PgConnectOptions {
        self.parse_connection_string().database(&self.database_name)
    }
}

pub fn get_app_mode() -> Result<Settings, ConfigError> {
    let base_path = env::current_dir().expect("failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let run_mode: Mode  = env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| {
            "development".into()
        })
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT");

    let settings = Config::builder()
        .add_source(File::from(configuration_directory.join("development")).required(true))
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .add_source(File::from(configuration_directory.join(run_mode.as_str())).required(true))
        .build()?;

    settings.try_deserialize()
}

pub enum Mode {
    Development,
    Production
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Development => "development",
            Self::Production => "production"
        }
    }
}

impl TryFrom<String> for Mode {
    type Error = String;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. use either development or production",
                other
            ))
        }
    }
}