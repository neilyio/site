#![allow(dead_code)]
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn canvas_context(canvas_id: &str) -> Result<CanvasRenderingContext2d, Object> {
    // Get canvas element by id
    let canvas = document()
        .get_element_by_id(canvas_id)
        .expect(&format!("no element with ID {canvas_id}"))
        .dyn_into::<HtmlCanvasElement>()
        .expect("element should be an HTMLCanvasElement");

    // Get the 2D context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>();

    context
}

pub fn animation_frame() -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        let closure = Closure::wrap(Box::new(move || {
            resolve.call0(&JsValue::null()).unwrap();
        }) as Box<dyn FnMut()>);

        window()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
        closure.forget();
    })
}
