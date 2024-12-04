mod home;
mod blog;

use home::HomeView;
use blog::BlogView;
use leptos::{component, view, ErrorBoundary, IntoView};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            log::error!("An error occurred: {:?}", errors);
            view! {
                <div>
                    Sorry an error occurred
                </div>
            }
        }>
            <Router>
                <Routes>
                    <Route path="/" view=HomeView />
                    <Route path="/:id" view=BlogView />
                </Routes>
            </Router>
            // <HomeView />
        </ErrorBoundary>
    }
}
