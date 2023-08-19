use crate::shapes;
use crate::shapes::Drawer;
use crate::utils;
use js_sys::Object;
use rand::{thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};
use utils::canvas_context;
use wasm_bindgen::prelude::*;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

pub fn ex1(canvas_id: &str) -> Result<(), JsValue> {
    let ctx = canvas_context(canvas_id)?;
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();
    let mut rng = thread_rng();

    let mut w = shapes::Dot {
        x: width as i32 / 2,
        y: height as i32 / 2,
        radius: 1,
        color: "red".to_string(),
    };

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    bg.draw(&ctx)?;
    utils::raf_loop(move || {
        let d = 4;

        for _ in 0..20 {
            // To step in any direction, including diagonal:
            w.x += rng.gen_range(-1..2) * d;
            w.y += rng.gen_range(-1..2) * d;

            // To step only up, down, left, right:
            // match rng.gen_range(0..4) {
            //     0 => w.x -= d,
            //     1 => w.x += d,
            //     2 => w.y -= d,
            //     _ => w.y += d,
            // }

            w.draw(&ctx)?;
        }

        Ok(())
    });

    Ok(())
}
