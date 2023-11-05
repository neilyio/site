mod api;
mod pages;
mod state;
mod utils;

use axum::{http::Response, response::IntoResponse, routing::get, Router};
use shared::Config;
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
    let config = Config::default();
    let state = state::AppState {
        config: config.clone(),
    };

    // Router setup.
    // The order here matters.
    let api_router = Router::new().route("/perlin2d", get(api::perlin2d::response));
    let app = Router::new()
        .route("/", get(pages::resume::response))
        .route("/resume", get(pages::resume::response))
        .route("/noc", get(pages::noc::response))
        .route("/style.css", get(style))
        // Layers only work on the routes declared before, so the
        // live reload layer should be declared before the api router.
        // This is because you don't want the live reload code to be
        // sent back with the HTML fragments.
        .layer(LiveReloadLayer::new())
        .nest("/api", api_router)
        .nest_service("/assets", ServeDir::new("server/assets"))
        .nest_service(
            &config.wasm_dir_endpoint,
            ServeDir::new(&config.wasm_target_dir_path),
        )
        // with_state must come after the nested route declarations.
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
