#![allow(dead_code, unused_imports)]
mod ex;
mod shapes;
mod utils;

use crate::shapes::Drawer;
use console_error_panic_hook;
use js_sys::Object;
use rand::{thread_rng, Rng};
use serde_json;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
use utils::canvas_context;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, Document, HtmlCanvasElement};

type CanvasFn = fn(CanvasRenderingContext2d) -> Result<(), JsValue>;

struct CanvasMap<'a> {
    map: HashMap<&'a str, CanvasFn>,
}

impl<'a> CanvasMap<'a> {
    fn new(entries: Vec<(&'a str, CanvasFn)>) -> Self {
        let mut cm = CanvasMap {
            map: HashMap::new(),
        };

        for entry in entries {
            cm.map.insert(entry.0, entry.1);
        }

        cm
    }

    fn init(&self, canvas_id: &str) -> () {
        let ctx = canvas_context(canvas_id).expect(&format!("No canvas found with id {canvas_id}"));

        match self.map.get(canvas_id) {
            Some(init_fn) => init_fn(ctx).unwrap(),
            None => (),
        }
    }
}

const ID_MAP: &[(&str, CanvasFn)] = &[
    ("0.1", ex::intro::ex1),
    ("0.2", ex::intro::ex2),
    ("0.3", ex::intro::ex3),
    ("0.4", ex::intro::ex4),
    ("0.5", ex::intro::ex5),
    ("0.6", ex::intro::ex6),
    ("1.1", ex::vectors::ex1),
];

#[allow(dead_code, unused_variables)]
#[wasm_bindgen]
pub fn run(filter_ids: String) -> Result<(), JsError> {
    console_error_panic_hook::set_once();

    let cm = CanvasMap::new(ID_MAP.to_vec());
    let ids: Vec<String> = serde_json::from_str(&filter_ids)?;

    for id in &ids {
        cm.init(id);
    }

    Ok(())
}
