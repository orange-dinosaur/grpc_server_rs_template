use serde::Deserialize;
use std::{env, str::FromStr, sync::OnceLock};
use tracing::Level;

use crate::error::{Error, Result};

// region: Environment

#[derive(Deserialize, Debug, PartialEq)]
pub enum Environment {
    Production,
    Development,
    Test,
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "production" => Ok(Environment::Production),
            "development" => Ok(Environment::Development),
            "test" => Ok(Environment::Test),
            _ => Err(Error::ConfigInvalidEnvironment(s.to_string())),
        }
    }
}

// endregion: Environment

// The config instance is initialized only once
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("Failed to load config from environment variables: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // Tracing
    pub TRACING_MAX_LEVEL: tracing::Level,

    // gRPC server auth credentials
    pub GRPC_AUTH_KEY: String,
    pub GRPC_AUTH_VALUE: String,
}

fn default_environment() -> Environment {
    Environment::Development
}

impl Config {
    fn load_from_env() -> Result<Config> {
        let environment = get_env("ENVIRONMENT").map_or_else(
            |_| default_environment(),
            |e| e.parse::<Environment>().unwrap_or(default_environment()),
        );

        let tracing_max_level = get_tracing_max_level(&environment)?;

        let grpc_auth_key = get_env("GRPC_AUTH_KEY")?;
        let grpc_auth_value = get_env("GRPC_AUTH_VALUE")?;

        Ok(Config {
            TRACING_MAX_LEVEL: tracing_max_level,

            GRPC_AUTH_KEY: grpc_auth_key,
            GRPC_AUTH_VALUE: grpc_auth_value,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_tracing_max_level(env: &Environment) -> Result<Level> {
    match env {
        Environment::Test => Ok(tracing::Level::TRACE),
        Environment::Development => Ok(tracing::Level::TRACE),
        Environment::Production => Ok(tracing::Level::INFO),
    }
}
