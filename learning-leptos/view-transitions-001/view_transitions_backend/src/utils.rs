use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::model::Config;

pub async fn init_db(cfg: &Config) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .after_connect(|conn, meta| Box::pin(async move {
    sqlx::query!("
        PRAGMA journal_mode = WAL;
        PRAGMA busy_timeout = 5000;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
    ").execute(conn).await;
            return Ok(())
        }))
        .connect(&cfg.db.url).await.unwrap();
    return pool;
}
