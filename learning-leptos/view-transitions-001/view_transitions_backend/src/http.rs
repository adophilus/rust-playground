use view_transitions_core::model::Blog;
use axum::{
    extract::{Json, Path, State},
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, Router},
};
use http::Method;
use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::model::Context;

async fn get_articles(State(ctx): State<Arc<Context>>) -> impl IntoResponse {
    let blogs = sqlx::query_as!(Blog, "SELECT * FROM blogs LIMIT 10")
        .fetch_all(&ctx.db.pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!(blogs)))
}

async fn get_article_by_id(
    State(ctx): State<Arc<Context>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(Blog, "SELECT * FROM blogs WHERE id = $1", id)
        .fetch_optional(&ctx.db.pool)
        .await
        .unwrap();

    match result {
        Some(blog) => (StatusCode::OK, Json(json!(blog))),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Blog not found!" })),
        ),
    }
}

pub async fn start_server(ctx: Context) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", ctx.app.port))
        .await
        .unwrap();
    let app = Router::new()
        .route("/api/articles", get(get_articles))
        .route("/api/articles/:id", get(get_article_by_id))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST])
                    .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>().unwrap()),
            ),
        )
        .with_state(Arc::new(ctx.clone()));

    log::info!("App running on port 0.0.0.0:{}", ctx.app.port);
    axum::serve(listener, app).await.unwrap();
}
