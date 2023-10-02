use askama::Template;
use axum::{extract::Query, http::Response, response::IntoResponse};
use hyper::Body;
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use shared::noise::perlin_2d_array;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Perlin2dParams {
    width: usize,
    height: usize,
}

pub async fn perlin2d(
    Query(Perlin2dParams { width, height }): Query<Perlin2dParams>,
) -> impl IntoResponse {
    let mut data = vec![0u8; 4 * width * height];
    let foreground = Vector4::new(255.0, 0.0, 0.0, 255.0);
    let background = Vector4::new(240.0, 255.0, 240.0, 255.0);
    let octaves: usize = 6;

    perlin_2d_array(&mut data, width, height, octaves, foreground, background);

    Response::builder()
        .header("content-type", "application/octet-stream")
        .body(Body::from(data))
        .unwrap()
}
