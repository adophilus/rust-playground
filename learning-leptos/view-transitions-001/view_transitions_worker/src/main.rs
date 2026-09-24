use serde::Deserialize;
use std::env;
use view_transitions_core::database::Database;

#[derive(Deserialize)]
struct GoogleBloggerApiV3PostsResposePostItemBlog {
    id: String,
}

#[derive(Deserialize)]
struct GoogleBloggerApiV3PostsResposePostItemAuthorImage {
    url: String,
}

#[derive(Deserialize)]
struct GoogleBloggerApiV3PostsResposePostItemReplies {
    #[serde(rename = "totalItems")]
    total_items: String,
    #[serde(rename = "selfLink")]
    self_link: String,
}

#[derive(Deserialize)]
struct GoogleBloggerApiV3PostsResposePostItemAuthor {
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
    url: String,
    image: GoogleBloggerApiV3PostsResposePostItemAuthorImage,
}

#[derive(Deserialize)]
struct GoogleBloggerApiV3PostsResposePostItem {
    kind: String,
    id: String,
    blog: GoogleBloggerApiV3PostsResposePostItemBlog,
    published: String,
    updated: String,
    url: String,
    #[serde(rename = "selfLink")]
    self_link: String,
    title: String,
    content: String,
    author: GoogleBloggerApiV3PostsResposePostItemAuthor,
    replies: GoogleBloggerApiV3PostsResposePostItemReplies,
    etag: String,
}

#[derive(Deserialize)]
struct GoogleBloggerApiV3PostsResponse {
    kind: String,
    #[serde(rename = "nextPageToken")]
    next_page_token: String,
    items: Vec<GoogleBloggerApiV3PostsResposePostItem>,
    etag: String,
}

async fn fetch_blog_posts(
    blogger_api_key: String,
    blog_id: String,
) -> GoogleBloggerApiV3PostsResponse {
    let res = reqwest::get(format!(
        "https://www.googleapis.com/blogger/v3/blogs/{blog_id}/posts?key={blogger_api_key}"
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    // log::debug!("Response from google blogger api v3 pos response endpoint: {res}");

    return serde_json::from_str(&res).unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("DATABASE_URL not set"));
    let blogger_api_key =
        env::var("BLOGGER_API_KEY").unwrap_or(String::from("BLOGGER_API_KEY not set"));
    let database = Database::init(database_url).await;

    fetch_blog_posts(blogger_api_key, String::from("2399953")).await;
}
