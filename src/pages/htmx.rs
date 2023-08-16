// use crate::utils;
use askama::Template;
use axum::{http::Response, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "htmx.html")]
pub struct HtmxPage {}

pub async fn response() -> impl IntoResponse {
    let page = HtmxPage {};

    Response::builder()
        .header("content-type", "text/html")
        .body(page.render().unwrap())
        .unwrap()
}
