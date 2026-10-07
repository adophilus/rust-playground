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
    pub async fn init(config: Config) -> Self {
        Self {
            db: Database::init(config.db.url).await,
            app: AppContext {
                port: config.app.port,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedMeta {
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

impl std::convert::From<sqlx::types::Json<PaginatedMeta>> for PaginatedMeta {
    fn from(value: sqlx::types::Json<PaginatedMeta>) -> Self {
        return value.0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedItems<T: Clone + std::fmt::Debug>(pub Vec<T>);

// impl<T: Clone + std::fmt::Debug + serde::de::DeserializeOwned> std::convert::From<String> for PaginatedItems<T> {
//     fn from(value: String) -> Self {
//         dbg!(&value);
//         return serde_json::from_str(&value).expect("Invalid PaginatedItems string");
//     }
// }

impl<T: Clone + std::fmt::Debug> std::convert::From<sqlx::types::Json<PaginatedItems<T>>> for PaginatedItems<T> {
    fn from(value: sqlx::types::Json<PaginatedItems<T>>) -> Self {
        return value.0;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T: Clone + std::fmt::Debug> {
    pub items: Vec<T>,
    pub meta: PaginatedMeta,
}

impl<T: Clone + std::fmt::Debug> Paginated<T> {
    pub fn new(items: Vec<T>, meta: PaginatedMeta) -> Self {
        return Self { items, meta };
    }
}

#[macro_export]
macro_rules! define_paginated {
    ($type: ty) => {
        paste::paste! {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            struct [<Paginated $type>] {
                items: PaginatedItems<$type>,
                meta: PaginatedMeta,
            }

            impl std::convert::From<[<Paginated $type>]> for Paginated<$type> {
                fn from(value: [<Paginated $type>]) -> Paginated<$type> {
                    return Paginated::new(value.items.0, value.meta);
                }
            }
        }
    };
}
