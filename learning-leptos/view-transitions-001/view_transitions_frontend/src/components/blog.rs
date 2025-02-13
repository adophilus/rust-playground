use crate::components::section::{Section, SectionCentral};
use leptos::{component, document, view, For, IntoView};
use leptos_router::{use_navigate, NavigateOptions};
use view_transitions_core::model::Blog;
use wasm_bindgen::{closure::Closure, JsCast};

#[component]
pub fn BlogTile(blog: Blog) -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <button on:click={
            let blog = blog.clone();

            move |_| {
                let navigate = navigate.clone();
                let callback = Closure::once_into_js({
                    let blog = blog.clone();
                    move || {
                        navigate(&format!("/{}", blog.id), NavigateOptions::default());
                    }
                });
                let transition = document().start_view_transition_with_update_callback(Some(callback.unchecked_ref())).unwrap();
            }
        }>
            <div class="tile" style=format!("--view-transition-name:blog-tile-{}", blog.id)>
                <img src=blog.image_url class="cover-image" style=format!("--view-transition-name:blog-image-{}", blog.id) / >
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
