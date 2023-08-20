#![allow(dead_code, unused_imports)]
mod ex;
mod shapes;
mod utils;

use crate::shapes::Drawer;
use js_sys::Object;
use rand::{thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};
use utils::canvas_context;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    ex::intro::ex2("myCanvas")?;

    Ok(())
}
