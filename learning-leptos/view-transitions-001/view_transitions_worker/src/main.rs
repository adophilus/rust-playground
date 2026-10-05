mod model;

use futures_util::{pin_mut, stream::StreamExt};
use model::{Blog, BlogManager, Config, LiveBlogManager};
use serde_json::json;
use std::error::Error;
use view_transitions_core::database::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::init()?;

    let database = Database::init(config.database_url).await;

    let posts_count = 1000;

    let blog_manager = LiveBlogManager::new(config.blogger_base_url, config.blogger_api_key);
    // let blog_manager = MockBlogManager::new();
    let blog = blog_manager.blog(String::from("2399953"));
    // let blog = blog_manager.blog();
    let blog_posts = blog.posts();

    let mut i = 0;

    pin_mut!(blog_posts);

    let mut tx = database.conn.begin().await.unwrap();

    while let Some(post) = blog_posts.next().await
        && i < posts_count
    {
        let blog_post = view_transitions_core::model::Blog::from(post);
        let blog_post_tags = json!(&blog_post.tags);

        sqlx::query_as!(
            view_transitions_core::model::Blog,
            "
        INSERT INTO
            blogs (id, title, content, cover_image_url, source_url, tags)
        VALUES
            (?, ?, ?, ?, ?, ?)
        RETURNING
            *
        ",
            blog_post.id,
            blog_post.title,
            blog_post.content,
            blog_post.cover_image_url,
            blog_post.source_url,
            blog_post_tags
        )
        .fetch_one(&mut *tx)
        .await?;

        log::info!("Seeded post {}...", i + 1);

        i += 1;
    }

    tx.commit().await.unwrap();

    log::info!("Done!");

    return Ok(());
}
