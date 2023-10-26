use askama::Template;
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "resume.html")]
pub struct ResumePage {
    pub me: Me,
    pub jobs: Vec<Jobs>,
    pub awards: Vec<Awards>,
    pub education: Vec<Education>,
}

#[derive(Deserialize)]
pub struct Me {
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub phone: String,
    pub email: String,
    pub location: String,
    pub bio: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Jobs {
    pub company: String,
    pub position: String,
    pub time_span: String,
    pub year_span: String,
    pub projects: Vec<String>,
    pub tech: Vec<String>,
}

#[derive(Deserialize)]
pub struct Awards {
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Education {
    pub title: String,
    pub subtitle: String,
    pub time_span: String,
    pub description: String,
}
