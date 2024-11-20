use std::env;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub content: String,
    pub link: String,
    pub image_url: String,
    pub topic: String,
    pub info: String,
    pub preprocessed: String,
}

#[derive(Clone)]
pub struct AppContext {
    pub port: u16,
}

#[derive(Clone)]
pub struct DatabaseContext {
    pub pool: SqlitePool,
    pub url: String,
}

#[derive(Clone)]
pub struct Context {
    pub db: DatabaseContext,
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
    pub fn from(config: Config, pool: SqlitePool) -> Self {
        Self {
            db: DatabaseContext {
                pool,
                url: config.db.url,
            },
            app: AppContext {
                port: config.app.port,
            },
        }
    }
}
