use view_transitions_core::model::Blog;

pub async fn get_blog_articles() -> Vec<Blog> {
    reqwest::get("http://127.0.0.1:8000/api/articles")
        .await
        .expect("Failed to get articles")
        .json()
        .await
        .expect("Failed to decode articles")
}

pub async fn get_blog_article(id: usize) -> Blog {
    reqwest::get(&format!("http://127.0.0.1:8000/api/articles/{}", id))
        .await
        .expect("Failed to get article")
        .json()
        .await
        .expect("Failed to decode article")
}
