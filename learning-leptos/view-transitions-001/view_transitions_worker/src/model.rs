use futures_core::stream::Stream;
use serde::Deserialize;
use std::{convert::From, env, fmt::{Display, Formatter}};

const MOCK_POST: &str = r#"
{
  "kind": "blogger#post",
  "id": "7531385161213212970",
  "blog": {
    "id": "2399953"
  },
  "published": "2020-05-20T16:53:00-07:00",
  "updated": "2020-05-20T16:58:18-07:00",
  "url": "http://blogger.googleblog.com/2020/05/a-better-blogger-experience-on-web.html",
  "selfLink": "https://www.googleapis.com/blogger/v3/blogs/2399953/posts/7531385161213212970",
  "title": "A better Blogger experience on the web",
  "content": "<div dir=\"ltr\" style=\"text-align: left;\" trbidi=\"on\">\n<span id=\"docs-internal-guid-83e698cd-7fff-3537-1fd4-931000cddafb\"></span><br />\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span id=\"docs-internal-guid-83e698cd-7fff-3537-1fd4-931000cddafb\"><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">Since 1999, millions of people have expressed themselves on Blogger. From detailed posts about almost every </span><a href=\"https://adamapples.blogspot.com/\" style=\"text-decoration-line: none;\"><span style=\"color: #1155cc; font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">apple variety</span></a><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"> you could ever imagine to a blog dedicated to </span><a href=\"https://howtoblog.krishnainfotron.com/\" style=\"text-decoration-line: none;\"><span style=\"color: #1155cc; font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">the art of blogging</span></a><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"> itself, the ability to easily share, publish and express oneself on the web is at the core of Blogger’s mission. As the web constantly evolves, we want to ensure anyone using Blogger has an easy and intuitive experience publishing their content to the web.</span><span style=\"font-family: arial; font-size: 11pt; font-style: italic; vertical-align: baseline; white-space: pre-wrap;\">&nbsp;&nbsp;</span></span></div>\n<span id=\"docs-internal-guid-83e698cd-7fff-3537-1fd4-931000cddafb\"><br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">That’s why we’ve been slowly introducing an improved web experience for Blogger. Give the fresh interface a spin by clicking “Try the New Blogger” in the left-hand navigation pane.&nbsp;</span></div><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\"><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><br /></span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt; text-align: center;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><span style=\"border: none; display: inline-block; overflow: hidden;\"><img src=\"https://lh5.googleusercontent.com/kWHfhyDmS0K6WMbTlfDV8Hq9RKq7Cs2sbPVl0otK3zDV5jNDO0SxM5-Ot89Wo3E11QvmNMI7VYMimqP-Vg9li-cz0cimWiGpJM65-uOSCmAvSN5n7M-lGcNWNW2u0cAfA54ZsGhZ\" style=\"margin-left: 0px; margin-top: 0px;\" width=\"508\" /></span></span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt; text-align: center;\">\n<span style=\"color: red; font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt; text-align: center;\">\n<span style=\"font-family: arial; font-size: 9pt; font-style: italic; vertical-align: baseline; white-space: pre-wrap;\">&nbsp;</span><span style=\"font-family: arial; font-size: 9pt; font-style: italic; vertical-align: baseline; white-space: pre-wrap;\">Click the “Try the New Blogger” button to see Blogger’s refreshed look and feel.</span></div>\n<br /><br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">In addition to a fresh feel, Blogger is now responsive on the web, making it easier to use on mobile devices. By investing in an improved web platform, it allows the potential for new features in the future.&nbsp;</span></div><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\"><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><br /></span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt; text-align: center;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><span style=\"border: none; display: inline-block; height: 299px; overflow: hidden; width: 146px;\"><img height=\"299\" src=\"https://lh4.googleusercontent.com/B-Tx1tl3m3_sH_4HiCg0XxhlTka0IV82jwT2LT4T9kzbXF15nMxjwNGe3NUAz-F42irNGdDINUiw4DM---nX_87Bb0X3OL_s5L19Rlyfhtm6oyEMNR1R4473TzkgsuxWQ3HXOIOV\" style=\"margin-left: 0px; margin-top: 0px;\" width=\"146\" /></span></span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt; text-align: center;\">\n<span style=\"font-family: arial; font-size: 8pt; vertical-align: baseline; white-space: pre-wrap;\"><span style=\"font-size: x-small;\"><font size=\"2\">Blogger’s new responsive design makes it easy to manage your blog on-the-go.</font></span></span></div>\n<br /><br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\"><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><br /></span></div><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">Learn more about the page-specific updates we’ve released to make your Blogger experience even better:&nbsp;</span></div>\n<br /><br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; font-weight: 700; vertical-align: baseline; white-space: pre-wrap;\">Stats</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">The redesigned Stats page helps you focus on the most important data from your blog by highlighting your most recent post.&nbsp;&nbsp;&nbsp;&nbsp;</span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; font-weight: 700; vertical-align: baseline; white-space: pre-wrap;\">Comments</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">A fresh Comments page helps you connect with readers more easily by surfacing areas that need your attention, like comment moderation.&nbsp;&nbsp;</span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; font-weight: 700; vertical-align: baseline; white-space: pre-wrap;\">Posts</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">We’ve improved support for </span><a href=\"https://support.google.com/blogger/answer/9675453?hl=en\" style=\"text-decoration-line: none;\"><span style=\"color: #1155cc; font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">Search Operators</span></a><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"> on the Posts page to help you filter your Blogger posts and page search results more easily.&nbsp;</span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; font-weight: 700; vertical-align: baseline; white-space: pre-wrap;\">Editor</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">The newly enhanced Editor page introduces table support, enables better transliteration, and includes an improved image/video upload experience.&nbsp;</span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; font-weight: 700; vertical-align: baseline; white-space: pre-wrap;\">Reading List&nbsp;</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">Even if you don’t create from your phone, it’s now easier than ever to read blogs from other creators while you’re on the go.</span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; font-weight: 700; vertical-align: baseline; white-space: pre-wrap;\">Settings&nbsp;</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">We’ve streamlined the Settings page to help you manage all your controls from one place.&nbsp;&nbsp;</span></div>\n<br /><br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">We’ll be moving everyone to the new interface over the coming months. Starting in late June, many Blogger creators will see the new interface become their default, though they can revert to the old interface by clicking “Revert to legacy Blogger” in the left-hand navigation. By late July, creators will no longer be able to revert to the legacy Blogger interface.&nbsp;&nbsp;</span></div>\n<br /><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\">We recommend getting ahead of the transition by opting into the experience today. Be sure to let us know what you think about the new design by tapping the Help icon in the top navigation bar. We can’t wait to see how Blogger creators use the latest updates to share their voice with the world.</span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><br /></span></div>\n<div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\"><span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><i><br /></i></span></div><div dir=\"ltr\" style=\"line-height: 1.38; margin-bottom: 0pt; margin-top: 0pt;\">\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><i>Posted by Fontaine on behalf of the Blogger team</i></span></div>\n<div>\n<span style=\"font-family: arial; font-size: 11pt; vertical-align: baseline; white-space: pre-wrap;\"><br /></span></div>\n</span></div>\n",
  "author": {
    "id": "04878303798219763289",
    "displayName": "A Googler",
    "url": "https://www.blogger.com/profile/04878303798219763289",
    "image": {
      "url": "//www.blogger.com/img/blogger_logo_round_35.png"
    }
  },
  "replies": {
    "totalItems": "0",
    "selfLink": "https://www.googleapis.com/blogger/v3/blogs/2399953/posts/7531385161213212970/comments"
  },
  "etag": "\"dGltZXN0YW1wOiAxNTkwMDE5MDk4ODE0Cm9mZnNldDogLTI1MjAwMDAwCg\""
}"#;


#[derive(Deserialize, Clone, Debug)]
pub struct GoogleBloggerApiV3PostsResponsePostItemBlog {
    pub id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleBloggerApiV3PostsResponsePostItemAuthorImage {
    pub url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleBloggerApiV3PostsResponsePostItemReplies {
    #[serde(rename = "totalItems")]
    pub total_items: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleBloggerApiV3PostsResponsePostItemAuthor {
    pub id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub url: Option<String>,
    pub image: GoogleBloggerApiV3PostsResponsePostItemAuthorImage,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleBloggerApiV3PostsResponsePostItem {
    pub kind: String,
    pub id: String,
    pub blog: GoogleBloggerApiV3PostsResponsePostItemBlog,
    pub published: String,
    pub updated: String,
    pub url: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
    pub title: String,
    pub content: String,
    pub author: GoogleBloggerApiV3PostsResponsePostItemAuthor,
    pub replies: GoogleBloggerApiV3PostsResponsePostItemReplies,
    pub etag: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GoogleBloggerApiV3PostsResponse {
    pub kind: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    pub items: Vec<GoogleBloggerApiV3PostsResponsePostItem>,
    pub etag: String,
}

pub struct LiveBlogManager {
    pub base_url: String,
    pub api_key: String,
    pub client: reqwest::Client,
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

pub struct MockBlogManager {}

impl MockBlogManager {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn blog(&self) -> impl Blog {
        return MockBlog::init();
    }
}

pub trait Blog {
    fn posts(&self) -> impl Stream<Item = GoogleBloggerApiV3PostsResponsePostItem>;
}

fn get_cover_image_url(post: &GoogleBloggerApiV3PostsResponsePostItem) -> Result<String, Error> {
        let dom = tl::parse(&post.content, tl::ParserOptions::default())?;
        let parser = dom.parser();
        let first_img = dom.query_selector("img").ok_or(Error{}).next().ok_or(Error{}).get(parser).ok_or(Error{}).as_tag().ok_or(Error{});

        let attributes = first_img.attributes();

        let id= attributes.get("id").ok_or(Error{});

        return attributes.get("src").ok_or(Error{});
}

impl From<T> for view_transitions_core::model::Blog where T:GoogleBloggerApiV3PostsResponsePostItem{
    fn from(value: T) -> Self {
        return Self {
            id: value.id,
            title: value.title,
            content: value.content,
            cover_image_url: get_cover_image_url(&value).ok(),
            source_url: value.url,
            tags: Vec::new()
        };
    }
}

pub struct MockBlog {}

impl MockBlog {
    pub fn init() -> Self {
        return Self {};
    }
}

impl Blog for MockBlog {
  fn posts(&self) -> impl Stream<Item = GoogleBloggerApiV3PostsResponsePostItem> {
        return async_stream::stream! {
            yield serde_json::from_str::<GoogleBloggerApiV3PostsResponsePostItem>(MOCK_POST).unwrap();
        };
    }
}

struct LiveBlog<'a> {
    blog_manager: &'a LiveBlogManager,
    id: String,
}

impl<'a> LiveBlog<'a> {
    pub fn init(blog_manager: &'a LiveBlogManager, id: String) -> Self {
        return Self { blog_manager, id };
    }

    pub async fn fetch_page(
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

pub struct Config {
    pub database_url: String,
    pub blogger_base_url: String,
    pub blogger_api_key: String,
}

impl Config {
    pub fn init() -> Result<Config, Box<dyn std::error::Error>> {
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
}

#[derive(Debug)]
pub struct Error {}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        return fmt.write_str("");
    }
}

impl std::error::Error for Error{}

impl From<T> for Error where T: tl::errors::ParseError {
    fn from(value: T) -> Self {
        return Self {};
    }
}
