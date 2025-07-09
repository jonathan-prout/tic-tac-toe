use axum::response::{Html, IntoResponse};
use axum::routing::get;
use loco_rs::prelude::*;
use std::fs;

async fn serve_index() -> impl IntoResponse {
    match fs::read_to_string("static/index.html") {
        Ok(content) => Html(content).into_response(),
        Err(_) => (
            axum::http::StatusCode::NOT_FOUND,
            "File not found"
        ).into_response(),
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(serve_index))
}
