use futures_core::stream::Stream;
use futures_util::{pin_mut, stream::StreamExt};
use serde::Deserialize;
use std::{error::Error,env};
use view_transitions_core::database::Database;

#[derive(Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResposePostItemBlog {
    id: String,
}

#[derive(Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResposePostItemAuthorImage {
    url: String,
}

#[derive(Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResposePostItemReplies {
    #[serde(rename = "totalItems")]
    total_items: String,
    #[serde(rename = "selfLink")]
    self_link: String,
}

#[derive(Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResposePostItemAuthor {
    id: Option<String>,
    #[serde(rename = "displayName")]
    display_name: String,
    url: Option<String>,
    image: GoogleBloggerApiV3PostsResposePostItemAuthorImage,
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponse {
    kind: String,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
    items: Vec<GoogleBloggerApiV3PostsResposePostItem>,
    etag: String,
}

trait BlogManager {
    fn blog(self: &Self, id: String) -> impl Blog;
}

struct LiveBlogManager {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl LiveBlogManager {
    fn new(base_url: String, api_key: String) -> Self {
        let client = reqwest::Client::new();

        return Self { base_url, api_key, client  };
    }
}

impl BlogManager for LiveBlogManager {
    fn blog(self: &Self, id: String) -> impl Blog {
        return LiveBlog::init(self.clone(), id);
    }
}

trait Blog {
    fn init(blog_manager: impl BlogManager, id: String) -> Self;
    fn posts(self: &Self) -> impl Stream<Item = GoogleBloggerApiV3PostsResposePostItem>;
}

struct LiveBlog {
    blog_manager: LiveBlogManager,
    id: String,
}

impl LiveBlog {
    async fn fetch_page(
        self: &Self,
        page_token: Option<String>,
    ) -> GoogleBloggerApiV3PostsResponse {
        let mut query = Vec::new();
        query.push(("key", self.blog_manager.api_key.clone()));

        if let Some(page_token) = page_token {
            query.push(("pageToken", page_token));
        }

        let res = self
            .blog_manager
            .client
            .get(format!(
                "{}/blogger/v3/blogs/{}/posts",
                self.blog_manager.base_url,
                self.id
            ))
            .query(&query)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        return serde_json::from_str(res.as_str()).unwrap();
    }
}

impl Blog for LiveBlog {
    fn init(blog_manager: LiveBlogManager, id: String) -> Self {
        return Self { blog_manager, id };
    }

    fn posts(self: &Self) -> impl Stream<Item = GoogleBloggerApiV3PostsResposePostItem> {
        let mut next_page_token: Option<String> = None;

        return async_stream::stream! {
            loop {
                let mut query = Vec::new();
                query.push(("key", self.blog_manager.api_key.clone()));

                if let Some(page_token) = next_page_token.clone() {
                    query.push(("pageToken", page_token));
                }

                let page = self.fetch_page(next_page_token).await;

                for post in page.items {
                    yield post;
                }

                if let Some(page_token) = page.next_page_token {
                    next_page_token = Some(page_token);
                }
                else {
                    break;
                }
            }
        };
    }
}

struct Config {
    database_url: String,
    blogger_base_url: String,
    blogger_api_key: String
}

fn get_config() -> Result<Config, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("DATABASE_URL not set"));
    let blogger_base_url = 
        env::var("BLOGGER_BASE_URL")?; // .unwrap_or(String::from("BLOGGER_BASE_URL not set"));
    let blogger_api_key =
        env::var("BLOGGER_API_KEY").unwrap_or(String::from("BLOGGER_API_KEY not set"));

    return Ok(Config { database_url, blogger_base_url, blogger_api_key });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    env_logger::init();

    let config = get_config()?;

    let _database = Database::init(config.database_url).await;

    let posts_count = 1;

    let blog_manager = LiveBlogManager::new(config.blogger_base_url, config.blogger_api_key);
    let blog = blog_manager.blog(String::from("2399953"));
    let blog_posts = blog.posts();

    let mut i = 0;

    pin_mut!(blog_posts);

    while let Some(post) = blog_posts.next().await {
        if i == posts_count {
            break;
        }

        i += 1;
    }

    log::info!("Done!");

    return Ok(());
}
