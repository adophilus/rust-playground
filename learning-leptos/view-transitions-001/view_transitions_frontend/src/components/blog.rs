use crate::components::section::{Section, SectionCentral};
use leptos::{component, document, view, For, IntoView};
use leptos_router::A;
use view_transitions_core::model::Blog;

#[component]
pub fn BlogTile(blog: Blog) -> impl IntoView {
    view! {
        <button on:click=|_| {
        document().start_view_transition();
    }>
            <div class="tile" style=format!("--view-transition-name:blog-tile-{}", blog.id)>
                <img src=blog.image_url class="cover-image" />
                <span class="overlay">
                    <span class="category"><span>{blog.topic}</span></span>
                    <span class="content">
                            <span class="title">{blog.title}</span>
                            <span class="details">
                                <span class="author">
                                    <img src="./assets/avatar.jpg" />
                                    <span>By Jane Doe</span>
                                </span>
                                <span class="date">22nd Nov, 2024</span>
                            </span>
                    </span>
                </span>
            </div>
        </button>
    }
}

#[component]
pub fn BlogArticlesGrid(blogs: Vec<Blog>) -> impl IntoView {
    view! {
        <Section>
            <SectionCentral>
                <div class="blog__grid">
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
