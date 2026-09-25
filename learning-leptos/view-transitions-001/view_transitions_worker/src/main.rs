use async_stream::stream;
use futures_core::stream::Stream;
use futures_util::{pin_mut, stream::StreamExt};
use serde::Deserialize;
use std::env;
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
    id: String,
    #[serde(rename = "displayName")]
    display_name: String,
    url: String,
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

struct BlogManager {
    client: reqwest::Client,
    api_key: String,
}

impl BlogManager {
    fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();

        return BlogManager { client, api_key };
    }

    fn blog(self: &Self, id: String) -> Blog {
        return Blog::init(self, id);
    }
}

struct Blog<'a> {
    blog_manager: &'a BlogManager,
    id: String,
}

impl<'a> Blog<'a> {
    fn init<'a>(blog_manager: &'a BlogManager, id: String) -> Self {
        return Blog { blog_manager, id };
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
                "https://www.googleapis.com/blogger/v3/blogs/{}/posts",
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

#[tokio::main]
async fn main() {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from("DATABASE_URL not set"));
    let blogger_api_key =
        env::var("BLOGGER_API_KEY").unwrap_or(String::from("BLOGGER_API_KEY not set"));
    let posts_count = 10;
    let _database = Database::init(database_url).await;

    let blog_manager = BlogManager::new(blogger_api_key);
    let blog = blog_manager.blog(String::from("2399953"));
    let blog_posts = blog.posts();

    let mut i = 0;

    pin_mut!(blog_posts);

    while let Some(post) = blog_posts.next().await {
        if i == posts_count {
            break;
        }

        log::info!("This is the post: {:?}", post);
        i += 1;
    }

    log::info!("Done!");
}
