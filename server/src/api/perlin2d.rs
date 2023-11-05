use axum::{
    extract::Query,
    response::{IntoResponse, Response},
};
use hyper::Body;
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use shared::noise::perlin_2d_array;

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    width: usize,
    height: usize,
}

pub async fn response(Query(Params { width, height }): Query<Params>) -> impl IntoResponse {
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
