// src/main.rs

// dependencies
use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

// main function, annotated for the shuttle_runtime as well as shuttle_static_folder
#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    // build the router
    let router = Router::new()
        // serve the web files up at the "/" route
        .nest_service("/", ServeDir::new(static_folder));

    Ok(router.into())
}
