use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::model::Config;

pub async fn init_db(ctx: &Config) -> SqlitePool {
    SqlitePoolOptions::new().connect(&ctx.db.url).await.unwrap()
}
