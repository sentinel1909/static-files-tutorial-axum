// src/main.rs

use axum::Router;
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn axum(
) -> shuttle_axum::ShuttleAxum {
    
    let router = Router::new()
        
        .nest_service("/", ServeDir::new(PathBuf::from("static")));

    Ok(router.into())
}
