mod routes;
mod state;

use axum::{Router, routing::get};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(routes::page::index))
        .route("/partials/metrics", get(routes::partials::metrics))
        .route("/partials/ports_watch", get(routes::partials::ports_watch))
        .route("/partials/top_procs", get(routes::partials::top_procs))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");

    println!("DevDash running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
