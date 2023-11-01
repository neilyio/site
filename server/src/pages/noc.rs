use askama::Template;
use axum::{extract::Query, http::Response, response::IntoResponse};
use hyper::Body;
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use shared::{noc::ExerciseIds, noise::perlin_2d_array};

#[derive(Deserialize)]
pub struct Params {
    f: Option<String>,
}

#[derive(Serialize, Deserialize, Template)]
#[template(path = "noc.html")]
pub struct Page {
    pub exercise_ids: ExerciseIds,
    pub js_path: String,
}

impl From<Params> for Page {
    fn from(params: Params) -> Self {
        let ex_str = params.f.unwrap_or_default();
        let filter_ids: Vec<String> = ex_str
            .trim_matches(',')
            .split(",")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let exercise_ids = if filter_ids.is_empty() {
            ExerciseIds::default()
        } else {
            ExerciseIds::from(filter_ids)
        };

        Page {
            exercise_ids,
            js_path: shared::config::WASM_JS_ENDPOINT.into(),
        }
    }
}

pub async fn response(Query(params): Query<Params>) -> impl IntoResponse {
    Response::builder()
        .header("content-type", "text/html")
        .body(Page::from(params).render().unwrap())
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
