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

struct BlogManager {
    api_key: String,
}

impl BlogManager {
    fn new(api_key: String) -> Self {
        return BlogManager { api_key };
    }

    fn blog(self: &Self, id: String) -> Blog {
        return Blog {
            blog_manager: Self,
            id,
        };
    }
}

struct Blog {
    blog_manager: BlogManager,
    id: String,
}

impl Blog {
    fn init(api_key: String, id: String) -> Self {
        return Blog { api_key, id };
    }

    fn posts(self: &Self) -> BlogPostIterator {
        return BlogPostIterator {
            blog: self.clone(),
            current_page_token: None,
            next_page_token: None,
            index: 0,
            posts: Vec::new(),
        };
    }
}

struct BlogPostIterator {
    blog: Blog,
    current_page_token: Option<String>,
    next_page_token: Option<String>,
    index: u64,
    posts: Vec<Posts>,
}

impl Iterator for BlogPostIterator {
    async fn get_page(
        self: &mut Self,
        page_token: Option<String>,
    ) -> GoogleBloggerApiV3PostsResponse {
        let client = reqwest::Client::new();
        let query = Vec::new();
        query.push(("key", self.api_key));

        if page_token.is_some() {
            query.push(("pageToken", page_token.unwrap()));
        }

        let res = client
            .get(format!(
                "https://www.googleapis.com/blogger/v3/blogs/{blog_id}/posts"
            ))
            .query(query)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // log::debug!("Response from google blogger api v3 pos response endpoint: {res}");

        return serde_json::from_str(&res).unwrap();
    }

    fn next() -> Option<Self::Item> {
        return None;
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("DATABASE_URL not set"));
    let blogger_api_key =
        env::var("BLOGGER_API_KEY").unwrap_or(String::from("BLOGGER_API_KEY not set"));
    let posts_count = 100;
    let database = Database::init(database_url).await;

    fetch_blog_posts(blogger_api_key, String::from("2399953")).await;
}
