use crate::state::AppState;
use crate::utils;
use askama_axum::Template;
use axum::extract::State;
use serde::Deserialize;
use shared::resume::{Awards, Education, Jobs, Me};

#[derive(Deserialize, Template)]
#[template(path = "resume.html")]
pub struct Page {
    pub me: Me,
    pub jobs: Vec<Jobs>,
    pub awards: Vec<Awards>,
    pub education: Vec<Education>,
}

pub async fn response(State(state): State<AppState>) -> Page {
    utils::parse_toml(&state.config.resume_toml_path).expect("error parsing resume toml")
}
