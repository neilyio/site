#![allow(dead_code)]
mod pages;
mod utils;

use axum::{http::Response, response::IntoResponse, routing::get, Router};
use notify::{FsEventWatcher, Watcher};
use std::path::Path;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

async fn style() -> impl IntoResponse {
    let template_files = utils::get_files_in_dir("templates").unwrap();

    Response::builder()
        .header("content-type", "text/css")
        .body(utils::build_css_classes(&template_files))
        .unwrap()
}

struct Reloader {
    layer: LiveReloadLayer,
    watcher: FsEventWatcher,
}

impl Reloader {
    fn new() -> Self {
        let layer = LiveReloadLayer::new();
        let reloader = layer.reloader();

        let mut watcher = notify::recommended_watcher(move |_| {
            reloader.reload();
        })
        .unwrap();

        watcher
            .watch(Path::new("templates"), notify::RecursiveMode::Recursive)
            .unwrap();

        Reloader { layer, watcher }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // APP SETUP
    let app = Router::new()
        .route("/", get(pages::resume::response))
        .route("/htmx", get(pages::htmx::response))
        .route("/style.css", get(style))
        .nest_service("/assets", ServeDir::new("assets"));

    let reloader = Reloader::new();
    #[cfg(debug_assertions)]
    let app = app.layer(reloader.layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
