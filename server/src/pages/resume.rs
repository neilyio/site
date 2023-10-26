use crate::utils;
use axum::{http::Response, response::IntoResponse};
use shared::pages::resume::ResumePage;
use shared::pages::Template;

pub async fn response() -> impl IntoResponse {
    let config = utils::parse_toml::<ResumePage>("server/data/data.toml").unwrap();

    let page = ResumePage {
        me: config.me,
        jobs: config.jobs,
        awards: config.awards,
        education: config.education,
    };
    Response::builder()
        .header("content-type", "text/html")
        .body(page.render().unwrap())
        .unwrap()
}
