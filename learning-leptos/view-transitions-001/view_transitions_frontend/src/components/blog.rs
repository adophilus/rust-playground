use crate::components::section::{Section, SectionCentral};
use leptos::{component, document, view, For, IntoView};
use leptos_router::{use_navigate, NavigateOptions};
use view_transitions_core::model::Blog;
use web_sys::js_sys::Function;

#[component]
pub fn BlogTile(blog: Blog) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <button
            class="blog__tile"
            style=format!("--view-transition-name:blog-tile-{}", blog.id)
            on:click={
                let blog = blog.clone();
                move |_| {
                    let navigate = navigate.clone();
                    navigate(&format!("/{}", blog.id), NavigateOptions::default());
                }
            }
        >
            <img
                // blog.cover_image_url.unwrap_or(String::from("https://placehold.co/500x500"))
                src="https://placehold.co/500x500"
                class="blog__cover-image"
                style=format!("--view-transition-name:blog-image-{}", blog.id)
            />
            <span class="blog__overlay">
                <span class="blog__category">
                    <span>tag</span>
                </span>
                <span class="blog__content">
                    <span class="blog__title">{blog.title}</span>
                    <span class="blog__details">
                        <span class="blog__author">
                            <img src="./assets/avatar.jpg" />
                            <span>By Jane Doe</span>
                        </span>
                        <span class="date">22nd Nov, 2024</span>
                    </span>
                </span>
            </span>
        </button>
    }
}

#[component]
pub fn BlogArticlesGrid(blogs: Vec<Blog>) -> impl IntoView {
    view! {
        <Section>
            <SectionCentral>
                <div class="blog__grid">
                    <header>A Simple View Transitions Demo</header>
                    <For
                        each=move || blogs.clone()
                        key=|blog| blog.id.clone()
                        children=move |blog: _| view! { <BlogTile blog /> }
                    />
                </div>
            </SectionCentral>
        </Section>
    }
}
