use reqwest::Client;
use view_transitions_core::model::{Blog, Paginated};

const API_ENDPOINT_URL: &str = "http://localhost:8000";

fn get_client() -> Client {
    Client::new()
}

pub async fn get_blog_articles() -> Paginated<Blog> {
    let query = [("page", 1), ("per_page", 10)];

    let client = get_client();
    client
        .get(format!("{}/api/articles", API_ENDPOINT_URL))
        .query(&query)
        .send()
        .await
        .expect("Failed to get articles")
        .json()
        .await
        .expect("Failed to decode articles")
}

pub async fn get_blog_article(id: usize) -> Blog {
    let client = get_client();

    client
        .get(format!("{}/api/articles/{}", API_ENDPOINT_URL, id))
        .send()
        .await
        .expect("Failed to get article")
        .json()
        .await
        .expect("Failed to decode article")
}
