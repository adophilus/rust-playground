use crate::components::section::{Section, SectionCentral};
use leptos::{component, view, IntoView, Params};
use leptos_router::{Params, use_params};

#[derive(Params, PartialEq)]
struct BlogParams {
    id: Option<usize>
}

#[component]
pub fn BlogView() -> impl IntoView {
    let params = use_params::<BlogParams>();

    view! {
        <div class="blog-page">
            <img src="https://via.assets.so/img.jpg?w=400&h=150&tc=#ffffff&bg=#cecece" class="__cover-image" />
            <Section>
                <SectionCentral>
                    <div>
                        <h1 class="__title">Blog title</h1>
                        <div class="__details">
                        </div>
                    </div>
                </SectionCentral>
            </Section>
        </div>
    }
}
