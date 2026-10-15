use std::cell::RefCell;

use crate::{
    components::section::{Section, SectionCentral},
    model::BlogQuery,
};
use js_sys::Promise;
use leptos::{component, document, view, For, IntoView};
use leptos_router::{use_navigate, NavigateOptions, ParamsMap};
use view_transitions_core::model::Blog;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};

fn next_frame() -> Promise {
    Promise::new(&mut |resolve, reject| {
        let resolve = resolve.clone();
        leptos::request_animation_frame(move || {
            resolve.call0(&JsValue::UNDEFINED);
        });
    })
}

async fn wait_for_view(selector: &str, timeout_ms: f64) {
    let start = js_sys::Date::now();

    loop {
        if leptos::document()
            .query_selector(selector)
            .expect("no querySelector")
            .is_some()
        {
            return ();
        } else if js_sys::Date::now() - start >= timeout_ms {
            return ();
        }
        JsFuture::from(next_frame()).await;
    }
}

#[component]
pub fn BlogTile(blog: Blog) -> impl IntoView {
    let navigate = use_navigate();
    let slot = RefCell::new(None);

    view! {
        <button
            class="blog__tile"
            style=format!("--view-transition-name: blog-tile-{}", blog.id)
            on:click={
                let blog = blog.clone();
                let blog_query_string = ParamsMap::try_from(BlogQuery(blog.clone()))
                    .unwrap()
                    .to_query_string();
                move |_| {
                    let navigate = navigate.clone();
                    let blog_id = blog.id.clone();
                    let blog_query_string = blog_query_string.clone();
                    let cb = Closure::<
                        dyn FnMut() -> Promise,
                    >::new(move || {
                        navigate(
                            &format!("/{}{}", &blog_id, &blog_query_string),
                            NavigateOptions::default(),
                        );
                        future_to_promise(async {
                            wait_for_view(".blog__page .blog__cover-image",2000.0).await;
                            Ok(JsValue::UNDEFINED)
                        })
                    });
                    let _ = document()
                        .start_view_transition_with_update_callback(
                            Some(cb.as_ref().unchecked_ref()),
                        );
                    *slot.borrow_mut() = Some(cb);
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
