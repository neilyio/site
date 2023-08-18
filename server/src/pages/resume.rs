#![allow(dead_code)]
use crate::utils;
use askama::Template;
use axum::{http::Response, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "resume.html")]
pub struct ResumePage {
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
