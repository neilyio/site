mod pages;
mod state;
mod utils;

use axum::{http::Response, response::IntoResponse, routing::get, Router};
use shared::config::{WASM_DIR_ENDPOINT, WASM_TARGET_DIR_PATH};
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

async fn style() -> impl IntoResponse {
    let template_files = utils::get_files_in_dir("server/templates").unwrap();

    Response::builder()
        .header("content-type", "text/css")
        .body(utils::build_css_classes(&template_files))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(state::AppState {
        count: Mutex::new(0),
    });

    // Router setup.
    // The order here matters.
    // with_state must come after the nested route declarations.
    // Layers only work on the routes declared before, so the
    // live reload layer should be declared before the api router.
    // This is because you don't want the live reload code to be
    // sent back with the HTML fragments.
    let api_router = Router::new().route("/perlin2d", get(pages::noc::perlin2d));
    let app = Router::new()
        .route("/", get(pages::resume::response))
        .route("/resume", get(pages::resume::response))
        .route("/noc", get(pages::noc::response))
        .route("/style.css", get(style))
        .layer(LiveReloadLayer::new())
        .nest("/api", api_router)
        .nest_service("/assets", ServeDir::new("server/assets"))
        .nest_service(WASM_DIR_ENDPOINT, ServeDir::new(WASM_TARGET_DIR_PATH))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
