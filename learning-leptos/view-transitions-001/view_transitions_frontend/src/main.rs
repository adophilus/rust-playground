mod components;
mod utils;
mod views;

use leptos::mount_to_body;
use views::App;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
