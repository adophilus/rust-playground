use async_stream::stream::Stream;
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
            blog_manager: self,
            id,
        };
    }
}

struct Blog {
    blog_manager: BlogManager,
    id: String,
}

impl Blog {
    fn init(blog_manager: BlogManager, id: String) -> Self {
        return Blog { blog_manager, id };
    }

    fn posts(self: &Self) -> Stream<Item = GoogleBloggerApiV3PostsResposePostItem> {
        let current_page_token: Option<String> = None;
        let next_page_token: Option<String> = None;
        let index: usize = 0;
        let mut blog_posts: Vec<GoogleBloggerApiV3PostsResposePostItem> = Vec::new();
        let client = reqwest::Client::new();
        let api_key = self.blog_manager.api_key;

        return async_stream::stream! {
            if index == posts.len() - 1 {
                let query = Vec::new();
                query.push(("key", self.blog_manager.api_key));

                if next_page_token.is_some() {
                    query.push(("pageToken", next_page_token.unwrap()));
                }

                let res = client
                    .get(format!(
                        "https://www.googleapis.com/blogger/v3/blogs/{slef.id}/posts"
                    ))
                    .query(query)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();


                // log::debug!("Response from google blogger api v3 pos response endpoint: {res}");

                let parsed = serde_json::from_str::<GoogleBloggerApiV3PostsResponse>(&res).unwrap();
                next_page_token = Some(parsed.next_page_token);
                blog_posts.extend_from_slice(parsed.items.as_slice());
            }

            index += 1;
            yield posts[index];
        };
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

    let blog_manager = BlogManager::new(blogger_api_key);
    let blog = blog_manager.blog(String::from("2399953"));
    let posts = blog.posts();

    let i = 0;
    for post in posts {
        if i == posts_count {
            break;
        }

        log::debug!(post);
        i += 1;
    }
}
