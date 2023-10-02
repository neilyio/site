#![allow(dead_code, unused_imports)]
mod ex;
mod shapes;
mod utils;

use crate::shapes::Drawer;
use futures::{future::join_all, TryFutureExt};
// use anyhow::Result;
use console_error_panic_hook;
use js_sys::Object;
use rand::{thread_rng, Rng};
use serde_json;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::{cell::RefCell, rc::Rc};
use utils::canvas_context;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::{console, CanvasRenderingContext2d, Document, HtmlCanvasElement};

// type CanvasFn = fn(CanvasRenderingContext2d) -> anyhow::Result<(), anyhow::Error>;
type CanvasFn = fn(
    CanvasRenderingContext2d,
) -> Pin<Box<dyn Future<Output = anyhow::Result<(), anyhow::Error>>>>;

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

    async fn init(&self, canvas_id: &str) -> () {
        let ctx = canvas_context(canvas_id).expect(&format!("No canvas found with id {canvas_id}"));

        match self.map.get(canvas_id) {
            Some(init_fn) => init_fn(ctx).await.unwrap(),
            None => (),
        }
    }
}

const ID_MAP: &[(&str, CanvasFn)] = &[
    ("0.1", |ctx| Box::pin(ex::intro::ex1(ctx))),
    ("0.2", |ctx| Box::pin(ex::intro::ex2(ctx))),
    ("0.3", |ctx| Box::pin(ex::intro::ex3(ctx))),
    ("0.4", |ctx| Box::pin(ex::intro::ex4(ctx))),
    ("0.5", |ctx| Box::pin(ex::intro::ex5(ctx))),
    ("0.6", |ctx| Box::pin(ex::intro::ex6(ctx))),
    ("1.1", |ctx| Box::pin(ex::vectors::ex1(ctx))),
];

#[allow(dead_code, unused_variables)]
#[wasm_bindgen]
pub fn run(filter_ids: String) -> js_sys::Promise {
    console_error_panic_hook::set_once();

    let future = async move {
        let cm = CanvasMap::new(ID_MAP.to_vec());
        let ids: Vec<String> = serde_json::from_str(&filter_ids).unwrap();

        let futures: Vec<_> = ids.iter().map(|id| cm.init(id)).collect();
        join_all(futures).await;

        Ok::<wasm_bindgen::JsValue, wasm_bindgen::JsValue>(JsValue::UNDEFINED)
    };

    future_to_promise(future.map_err(JsValue::from))
}
