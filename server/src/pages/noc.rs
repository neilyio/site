use askama::Template;
use axum::{extract::Query, http::Response, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Template)]
#[template(path = "noc.html")]
pub struct NocPage {
    filter_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    f: Option<String>,
}

const ALL_IDS: &[&str] = &["0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "1.1"];

pub async fn response(Query(Params { f }): Query<Params>) -> impl IntoResponse {
    let ex_str = f.unwrap_or_default();
    let filter_ids: Vec<String> = ex_str
        .trim_matches(',')
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let all_ids: Vec<String> = ALL_IDS.iter().map(|s| s.to_string()).collect();

    let page = NocPage {
        filter_ids: if filter_ids.is_empty() {
            all_ids
        } else {
            filter_ids
        },
    };

    Response::builder()
        .header("content-type", "text/html")
        .body(page.render().unwrap())
        .unwrap()
}
