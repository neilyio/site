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

pub fn ex2(canvas_id: &str) -> Result<(), JsValue> {
    let ctx = canvas_context(canvas_id)?;
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().width();
    let mut numbers: [i32; 20] = [0; 20];
    let mut rnd = thread_rng();

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    bg.draw(&ctx)?;
    utils::raf_loop(move || {
        // Increment a random number in the list.
        let index = rnd.gen_range(0..numbers.len());
        numbers[index] += 1;

        let bar_width = width / numbers.len() as u32;

        for (i, n) in numbers.iter().enumerate() {
            let rect = shapes::Rectangle {
                x: i as i32 * bar_width as i32,
                y: height as i32,
                width: bar_width as i32,
                height: *n * -1,
                color: "red".to_string(),
                border_color: "black".to_string(),
                border_width: 2,
            };

            rect.draw(&ctx)?;
        }

        Ok(())
    });
    Ok(())
}
