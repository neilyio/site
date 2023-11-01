#![allow(stable_features)]
#![feature(async_fn_in_trait)]
mod ex;
mod shapes;
mod sketch;
mod utils;

use anyhow::Result;
use ex::{intro, vectors};
use gloo::console;
use shared::noc;
use sketch::Render;
use std::convert::TryFrom;
use utils::canvas_context;
use wasm_bindgen::prelude::*;

#[allow(dead_code, unused_variables)]
#[wasm_bindgen]
pub fn run(input: JsValue) -> Result<(), JsError> {
    console_error_panic_hook::set_once();

    let exercise_ids = noc::ExerciseIds::try_from(input).unwrap();

    for id in exercise_ids {
        let ctx = canvas_context(&id).expect(&format!("Couldn't find element with id: {id}"));
        let result = match id.as_ref() {
            "0.1" => intro::Ex1::render(ctx),
            "0.2" => intro::Ex2::render(ctx),
            "0.3" => intro::Ex3::render(ctx),
            "0.4" => intro::Ex4::render(ctx),
            "0.5" => intro::Ex5::render(ctx),
            "0.6" => intro::Ex6::render(ctx),
            "1.1" => vectors::Ex1::render(ctx),
            _ => Err(anyhow::format_err!("No Exercise with id: {id}")),
        };

        if let Err(error) = result {
            console::error!(format!("Rendering error: {error:#?}"));
        }
    }

    Ok(())
}
