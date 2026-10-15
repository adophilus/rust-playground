use crate::views;
use leptos::{component, ev, view, window, window_event_listener, ErrorBoundary, IntoView};
use leptos_router::{use_navigate, NavigateOptions};

// This component is necessary for now cos it's like a patch work for the RouterContext not found
// issue encountered when you try to navigate back from an existing route. The issue is present in
// leptos_router v0.6.15, but I beleive it has been fixed in v0.7, so until I upgrade, this is a
// monkeypatch fix
#[component]
fn PopstateFix() -> impl IntoView {
    let navigate = use_navigate();

    window_event_listener(ev::popstate, move |_| {
        let url = window().location().href().unwrap();
        navigate(
            &url,
            NavigateOptions {
                resolve: false,
                replace: true,
                scroll: true,
                ..Default::default()
            },
        );
    });
}

#[component]
pub fn Router() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            log::error!("An error occurred: {:?}", errors);
            view! { <div>Sorry an error occurred</div> }
        }>
            <leptos_router::Router>
                <PopstateFix />
                <leptos_router::Routes>
                    <leptos_router::Route path="" view=views::blogs::BlogsView />
                    <leptos_router::Route path=":id" view=views::blog_by_id::BlogByIdView />
                </leptos_router::Routes>
            </leptos_router::Router>
        </ErrorBoundary>
    }
}
