mod components;
mod utils;
mod views;

use leptos::{mount_to_body, view};
use log::Level;
use views::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();

    mount_to_body(|| view! { <App />});
}
