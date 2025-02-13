use crate::{
    components::section::{Section, SectionCentral},
    utils,
};
use leptos::{component, create_resource, view, IntoView, Params, Show};
use leptos_reactive::SignalGet;
use leptos_router::{use_params, Params};
use view_transitions_core::model::Blog;

#[component]
fn BlogContent(blog: Blog) -> impl IntoView {
    view! {
        <div class="blog-page">
            <img src=blog.image_url class="cover-image" style=format!("--view-transition-name:blog-image-{}", blog.id) />
            <Section>
                <SectionCentral>
                    <div>
                        <h1 class="title">{blog.title}</h1>
                        <div class="details">
                            {blog.content}
                        </div>
                    </div>
                </SectionCentral>
            </Section>
        </div>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
struct BlogParams {
    id: Option<usize>,
}

#[component]
pub fn BlogView() -> impl IntoView {
    let params = use_params::<BlogParams>();
    let blog_id = params.get().unwrap().id.unwrap();
    let blog = create_resource(
        || (),
        {
            let blog_id = blog_id.clone();
            move |_| async move { utils::get_blog_article(blog_id).await }
        }
    );

    view! {
        <Show
            when=move || blog.get().is_some()
            fallback=|| view! {<div>Loading...</div>}
        >
            <BlogContent blog=blog.get().unwrap() />
        </Show>
    }
}
