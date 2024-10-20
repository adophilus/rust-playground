use leptos::*;
use std::marker::PhantomData;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| {
            set_count.update(|c| *c += 1)
        }>
        "Click me: "
        {move || count()}
        </button>
    }
}
