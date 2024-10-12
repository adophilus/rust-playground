use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use serde_json::json;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use vercel_runtime::{
    process_request, process_response, run, run_service, Body, Error, Request, Response,
    ServiceBuilder, StatusCode,
};
mod utils;
use utils::LambdaLayer;

struct AppContext {
    base_name: String,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            base_name: String::from("axum-on-rust"),
        }
    }
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

async fn get_index_route() -> impl IntoResponse {
    "Welcome to the api"
}

async fn get_greeting(Path(name): Path<String>) -> impl IntoResponse {
    format!("Greetings {}", name)
}

fn get_router() -> Router<Arc<AppContext>> {
    Router::new()
        .route("/", get(get_index_route))
        .route("/greet/:name", get(get_greeting))
}

use rand::seq::SliceRandom;

pub fn choose_starter() -> String {
    let pokemons = vec!["Bulbasaur", "Charmander", "Squirtle", "Pikachu"];
    let starter = pokemons.choose(&mut rand::thread_rng()).unwrap();
    starter.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let starter = choose_starter();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": format!("I choose you, {}!", starter),
            })
            .to_string()
            .into(),
        )?)
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let ctx = Arc::new(AppContext::new());
//
//     init_tracing();
//
//     let app = Router::new()
//         .nest("/api", get_router())
//         .with_state(ctx.clone())
//         .layer(TraceLayer::new_for_http());
//
//     let handler = ServiceBuilder::new()
//         .map_request(process_request)
//         .map_response(process_response)
//         .layer(LambdaLayer::default())
//         .service(app);
//
//     run_service(handler).await
// }
