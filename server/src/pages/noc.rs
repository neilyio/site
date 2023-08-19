use askama::Template;
use axum::{http::Response, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "noc.html")]
pub struct NocPage {}

pub async fn response() -> impl IntoResponse {
    let page = NocPage {};

    Response::builder()
        .header("content-type", "text/html")
        .body(page.render().unwrap())
        .unwrap()
}
