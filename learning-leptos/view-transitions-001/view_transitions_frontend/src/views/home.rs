use crate::components::blog::BlogArticlesGrid;
use crate::utils;
use leptos::{component, create_resource, view, IntoView, Show, SignalGet};

#[component]
pub fn HomeView() -> impl IntoView {
    let blogs = create_resource(|| (), |_| async move { utils::get_blog_articles().await });

    view! {
        <Show
            when=move|| blogs.get().is_some()
            fallback=|| view! { <div>Loading...</div>}
        >
            <BlogArticlesGrid blogs=blogs.get().unwrap() />
        </Show>
    }
}
