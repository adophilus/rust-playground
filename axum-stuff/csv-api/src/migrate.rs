use crate::model::{Blog, Context};
use async_std::stream::StreamExt;

pub async fn migrate(ctx: Context) {
    let raw_file = async_std::fs::File::open("./assets/preprocessed_blogs.csv")
        .await
        .unwrap();

    let reader = csv_async::AsyncReader::from_reader(raw_file);
    let mut records = reader.into_records();

    let mut tx = ctx.db.pool.begin().await.unwrap();

    while let Some(Ok(record)) = records.next().await {
        let blog: Blog = record.deserialize(None).unwrap();

        sqlx::query!(
            "
                INSERT INTO blogs (id, title, content, link, image_url, topic, info, preprocessed)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
            blog.id,
            blog.title,
            blog.content,
            blog.link,
            blog.image_url,
            blog.topic,
            blog.info,
            blog.preprocessed,
        )
        .execute(&mut *tx)
        .await
        .unwrap();
    }

    tx.commit().await.unwrap();

    log::info!("All done");
}
