use leptos::*;
use std::marker::PhantomData;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <textarea
            prop:value=move || some_value.get()
            on:input=/* etc */
        >
            /* plain-text initial value, does not change if the signal changes */
            {some_value.get_untracked()}
        </textarea>
    }
}
