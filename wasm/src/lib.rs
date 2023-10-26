#![allow(stable_features)]
#![feature(async_fn_in_trait)]
mod ex;
mod shapes;
mod sketch;
mod utils;

use crate::ex::intro;
use crate::ex::vectors;
use crate::sketch::Render;
use crate::utils::canvas_context;
use anyhow::Result;
use once_cell::sync::Lazy;
use serde_wasm_bindgen;
use shared::pages::noc;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

type RenderFn = fn(CanvasRenderingContext2d) -> Result<(), anyhow::Error>;
static RENDER: Lazy<Vec<(&'static str, RenderFn)>> = Lazy::new(|| {
    vec![
        ("0.1", intro::Ex1::render),
        ("0.2", intro::Ex2::render),
        ("0.3", intro::Ex3::render),
        ("0.4", intro::Ex4::render),
        ("0.5", intro::Ex5::render),
        ("0.6", intro::Ex6::render),
        ("1.1", vectors::Ex1::render),
    ]
});

#[allow(dead_code, unused_variables)]
#[wasm_bindgen]
pub fn run(wasm_data: JsValue) -> Result<(), JsError> {
    console_error_panic_hook::set_once();
    let data: noc::Data =
        serde_wasm_bindgen::from_value(wasm_data).expect("Couldn't deserialize wasm_data.");

    let filter_ids = data.filter_ids;
    let render_map: HashMap<String, RenderFn> =
        RENDER.iter().map(|&(id, rf)| (id.to_owned(), rf)).collect();

    for id in filter_ids {
        match (render_map.get(&id), canvas_context(&id)) {
            (Some(render_fn), Ok(ctx)) => render_fn(ctx).unwrap(),
            _ => {}
        }
    }

    Ok(())
}
