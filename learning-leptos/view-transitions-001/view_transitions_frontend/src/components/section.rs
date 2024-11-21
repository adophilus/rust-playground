use leptos::{component, view, Children, IntoView};

#[component]
pub fn SectionCentral(children: Children) -> impl IntoView {
    view! { <div class="section__central">{children()}</div> }
}

#[component]
pub fn Section(children: Children) -> impl IntoView {
    view! { <section class="section">{children()}</section> }
}
