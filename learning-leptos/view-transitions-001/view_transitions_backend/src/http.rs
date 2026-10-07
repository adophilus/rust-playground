use crate::{
    model::{Context, PaginationQuery},
    repo,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, Router},
};
use http::Method;
use serde_json::json;
use std::{fmt::Debug, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use view_transitions_core::model::Blog;

#[derive(Debug)]
struct Error {
    message: String,
    status: StatusCode,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        return (self.status, Json(json!({ "message": self.message }))).into_response();
    }
}

#[axum::debug_handler]
async fn get_articles(
    State(ctx): State<Arc<Context>>,
    Query(pagination_query): Query<PaginationQuery>,
) -> impl IntoResponse {
    let page = pagination_query.page;
    let per_page = pagination_query.per_page;

    return repo::list_blogs(&ctx.db.conn, page as i32, per_page as i32)
        .await
        .map(|v| Json(json!(v)))
        .map_err(|e| Error {
            message: e.message,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        });
}

async fn get_article_by_id(
    State(ctx): State<Arc<Context>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    return repo::get_blog_by_id(&ctx.db.conn, &id)
        .await
        .map(|v| Json(json!(v)))
        .map_err(|e| Error {
            message: e.message,
            status: StatusCode::INTERNAL_SERVER_ERROR,
        });
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
