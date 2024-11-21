mod components;
mod utils;

use crate::components::section::{Section, SectionCentral};
use chrono::NaiveDateTime;
use leptos::{component, create_resource, mount_to_body, view, IntoView};
use log::Level;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiResponsePagination {
    limit: usize,
    offset: usize,
    count: usize,
    total: usize,
}

#[derive(Deserialize, Debug)]
struct ApiResponseBlogPreview {
    author: Option<String>,
    title: String,
    description: String,
    url: String,
    source: String,
    image: Option<String>,
    category: String,
    language: String,
    country: String,
    published_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    pagination: ApiResponsePagination,
    data: Vec<ApiResponseBlogPreview>,
}

#[component]
fn BlogArticles() -> impl IntoView {
    let blogs = create_resource(
        || (),
        |value| async move {
            let res: ApiResponse = reqwest::get("http://127.0.0.1:8000/api/articles")
                .await
                .expect("Failed to get articles")
                .json()
                .await
                .expect("Failed to decode articles");
            log::info!("{:?}", res);
        },
    );

    view! {
        <Section>
            <SectionCentral>hi there :wave</SectionCentral>
        </Section>
    }
}

#[component]
fn App() -> impl IntoView {
    view! { <BlogArticles /> }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug);

    mount_to_body(|| App())
}
