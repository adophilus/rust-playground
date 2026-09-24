use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Executor;

pub struct Database {
    conn: SqlitePool,
}

impl Database {
    pub async fn init(database_url: String) -> Self {
        let conn = SqlitePoolOptions::new()
            .after_connect(|conn, meta| {
                Box::pin(async move {
                    conn.execute(
                        "
                        PRAGMA journal_mode = WAL;
                        PRAGMA busy_timeout = 5000;
                        PRAGMA synchronous = NORMAL;
                        PRAGMA foreign_keys = ON;
                        ",
                    )
                    .await?;
                    return Ok(());
                })
            })
            .connect(&database_url)
            .await
            .unwrap();

        return Database { conn };
    }
}
