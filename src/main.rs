// src/main.rs

// dependencies
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::path::PathBuf;
use tower_http::services::ServeDir;

// home handler which returns some simple HTML
async fn home() -> impl IntoResponse {
    Html("<h1>Static Files - served with Axum</h1>")
}

// main function, annotated for the shuttle_runtime as well as shuttle_static_folder
#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    // router
    let router = Router::new()
        // home endpoin
        .route("/", get(home))
        // /static endpoint, serves index.html, made available in the static folder
        .nest_service("/static", ServeDir::new(static_folder));

    Ok(router.into())
}
