#![allow(dead_code)]
use askama::Template;
use axum::{http::Response, response::IntoResponse, routing::get, Router};
use notify::{FsEventWatcher, Watcher};
use railwind::{parse_to_string, Source, SourceOptions};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tower_livereload::LiveReloadLayer;

#[derive(Deserialize, Template)]
#[template(path = "hello.html")]
struct Config {
    me: Me,
    jobs: Vec<Jobs>,
    awards: Vec<Awards>,
    education: Vec<Education>,
}

#[derive(Deserialize)]
struct Me {
    first_name: String,
    last_name: String,
    title: String,
    phone: String,
    email: String,
    location: String,
    bio: String,
}

#[derive(Deserialize)]
struct Jobs {
    company: String,
    position: String,
    time_span: String,
    year_span: String,
    projects: Vec<String>,
    tech: Vec<String>,
}

#[derive(Deserialize)]
struct Awards {
    title: String,
    subtitle: String,
    description: String,
}

#[derive(Deserialize)]
struct Education {
    title: String,
    subtitle: String,
    time_span: String,
    description: String,
}

struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        User { name }
    }
}

fn get_files_in_dir(dir: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(|s| s.to_str());
        if path.is_file() && extension == Some("html") {
            file_paths.push(path);
        }
    }

    Ok(file_paths)
}

fn build_css_classes(paths: &Vec<PathBuf>) -> String {
    let mut warnings = vec![];
    let sources = Source::Files(
        paths
            .iter()
            .map(|pb| SourceOptions {
                input: &pb,
                option: railwind::CollectionOptions::Html,
            })
            .collect(),
    );

    parse_to_string(sources, true, &mut warnings)
}

async fn root() -> impl IntoResponse {
    let config = parse_toml("data/data.toml").unwrap();

    let hello = Config {
        me: config.me,
        jobs: config.jobs,
        awards: config.awards,
        education: config.education,
    };
    Response::builder()
        .header("content-type", "text/html")
        .body(hello.render().unwrap())
        .unwrap()
}

async fn style() -> impl IntoResponse {
    let template_files = get_files_in_dir("templates").unwrap();

    Response::builder()
        .header("content-type", "text/css")
        .body(build_css_classes(&template_files))
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

fn parse_toml(path: &str) -> Result<Config, Box<dyn Error>> {
    let mut file = fs::File::open(Path::new(path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let value: Config = toml::from_str(&contents)?;

    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // APP SETUP
    let app = Router::new()
        .route("/", get(root))
        .route("/style.css", get(style));

    let reloader = Reloader::new();
    #[cfg(debug_assertions)]
    let app = app.layer(reloader.layer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
