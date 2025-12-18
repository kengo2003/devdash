use askama::Template;
use axum::{extract::State, response::Html};
use std::sync::Arc;

use crate::state::AppState;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    platform: &'static str,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let t = IndexTemplate {
        title: "DevDash".to_string(),
        platform: state.platform.as_str(),
    };
    Html(t.render().unwrap())
}
