use futures_core::stream::Stream;
use futures_util::{pin_mut, stream::StreamExt};
use serde::Deserialize;
use std::{default::Default, env, error::Error};
use view_transitions_core::database::Database;

#[derive(Default, Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponsePostItemBlog {
    id: String,
}

#[derive(Default, Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponsePostItemAuthorImage {
    url: String,
}

#[derive(Default, Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponsePostItemReplies {
    #[serde(rename = "totalItems")]
    total_items: String,
    #[serde(rename = "selfLink")]
    self_link: String,
}

#[derive(Default, Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponsePostItemAuthor {
    id: Option<String>,
    #[serde(rename = "displayName")]
    display_name: String,
    url: Option<String>,
    image: GoogleBloggerApiV3PostsResponsePostItemAuthorImage,
}

#[derive(Default, Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponsePostItem {
    kind: String,
    id: String,
    blog: GoogleBloggerApiV3PostsResponsePostItemBlog,
    published: String,
    updated: String,
    url: String,
    #[serde(rename = "selfLink")]
    self_link: String,
    title: String,
    content: String,
    author: GoogleBloggerApiV3PostsResponsePostItemAuthor,
    replies: GoogleBloggerApiV3PostsResponsePostItemReplies,
    etag: String,
}

#[derive(Default, Deserialize, Clone, Debug)]
struct GoogleBloggerApiV3PostsResponse {
    kind: String,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
    items: Vec<GoogleBloggerApiV3PostsResponsePostItem>,
    etag: String,
}

struct LiveBlogManager {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl LiveBlogManager {
    fn new(base_url: String, api_key: String) -> Self {
        let client = reqwest::Client::new();

        return Self {
            base_url,
            api_key,
            client,
        };
    }

    fn blog(&self, id: String) -> impl Blog {
        return LiveBlog::init(self, id);
    }
}

struct MockBlogManager {}

impl MockBlogManager {
    fn new() -> Self {
        return Self {};
    }

    fn blog(&self, id: String) -> impl Blog {
        return MockBlog::init();
    }
}

trait Blog {
    fn posts(&self) -> impl Stream<Item = GoogleBloggerApiV3PostsResponsePostItem>;
}

struct MockBlog {}

impl MockBlog {
    fn init() -> Self {
        return Self {};
    }
}

impl Blog for MockBlog {
    fn posts(&self) -> impl Stream<Item = GoogleBloggerApiV3PostsResponsePostItem> {
        return async_stream::stream! {
            yield GoogleBloggerApiV3PostsResponsePostItem::default();
        };
    }
}

struct LiveBlog<'a> {
    blog_manager: &'a LiveBlogManager,
    id: String,
}

impl<'a> LiveBlog<'a> {
    fn init(blog_manager: &'a LiveBlogManager, id: String) -> Self {
        return Self { blog_manager, id };
    }

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
                self.blog_manager.base_url, self.id
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

impl<'a> Blog for LiveBlog<'a> {
    fn posts(self: &Self) -> impl Stream<Item = GoogleBloggerApiV3PostsResponsePostItem> {
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
    blogger_api_key: String,
}

fn get_config() -> Result<Config, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("DATABASE_URL not set"));
    let blogger_base_url = env::var("BLOGGER_BASE_URL")?; // .unwrap_or(String::from("BLOGGER_BASE_URL not set"));
    let blogger_api_key =
        env::var("BLOGGER_API_KEY").unwrap_or(String::from("BLOGGER_API_KEY not set"));

    return Ok(Config {
        database_url,
        blogger_base_url,
        blogger_api_key,
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = get_config()?;

    let _database = Database::init(config.database_url).await;

    let posts_count = 1;

    // let blog_manager = LiveBlogManager::new(config.blogger_base_url, config.blogger_api_key);
    let blog_manager = MockBlogManager::new();
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
