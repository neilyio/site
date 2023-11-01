use crate::utils;
use askama::Template;
use axum::{http::Response, response::IntoResponse};
use serde::Deserialize;
use shared::resume::{Awards, Education, Jobs, Me};
use std::error::Error;

#[derive(Deserialize, Template)]
#[template(path = "resume.html")]
pub struct Page {
    pub me: Me,
    pub jobs: Vec<Jobs>,
    pub awards: Vec<Awards>,
    pub education: Vec<Education>,
}

impl TryFrom<&str> for Page {
    type Error = Box<dyn Error>;

    fn try_from(file_path: &str) -> Result<Self, Self::Error> {
        utils::parse_toml::<Page>(file_path)
    }
}

pub async fn response() -> impl IntoResponse {
    let page = Page::try_from("server/data/data.toml").unwrap();

    Response::builder()
        .header("content-type", "text/html")
        .body(page.render().unwrap())
        .unwrap()
}
