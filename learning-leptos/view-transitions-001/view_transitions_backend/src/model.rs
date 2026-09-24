use std::env;

use serde::{Deserialize, Serialize};
use view_transitions_core::database::Database;

#[derive(Clone)]
pub struct AppContext {
    pub port: u16,
}

#[derive(Clone)]
pub struct Context {
    pub db: Database,
    pub app: AppContext,
}

#[derive(Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
}

#[derive(Clone)]
pub struct Config {
    pub db: DatabaseConfig,
    pub app: AppConfig,
}

impl Config {
    pub fn new() -> Self {
        let port = env::var("PORT").unwrap_or(String::from("8000"));
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");

        Self {
            db: DatabaseConfig { url },
            app: AppConfig {
                port: port.parse().unwrap_or(8000),
            },
        }
    }
}

impl Context {
    pub fn from(config: Config) -> Self {
        Self {
            db: Database::init(config.db.url),
            app: AppContext {
                port: config.app.port,
            },
        }
    }
}
