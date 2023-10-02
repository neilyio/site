use crate::renderer::CanvasRenderer;
use crate::renderer::Drawable;
use derive_builder::Builder;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[derive(Builder, Default)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
    #[builder(setter(into))]
    pub color: String,
    pub radius: i32,
    #[builder(default = "1.0")]
    pub opacity: f64,
}

#[derive(Builder, Default)]
pub struct Background {
    pub color: String,
}

#[derive(Builder, Default)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    #[builder(setter(into))]
    pub color: String,
    #[builder(setter(into))]
    pub border_color: String,
}
