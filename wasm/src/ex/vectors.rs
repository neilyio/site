use crate::{
    shapes::{self, Drawer},
    utils::{self, canvas_context},
};
use anyhow::Result;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::CanvasRenderingContext2d;

pub async fn ex1(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().height();
    let height = ctx.canvas().unwrap().width();

    let mut ball = shapes::Dot {
        x: width as i32 / 2,
        y: height as i32 / 2,
        radius: 32,
        color: "red".to_string(),
        opacity: 1.0,
    };

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    let mut x_speed = 10.0;
    let mut y_speed = 15.0;

    utils::raf_loop(move || {
        ball.x += x_speed as i32;
        ball.y += y_speed as i32;

        if ball.x < 0 || ball.x > width as i32 {
            x_speed *= -1.0
        }

        if ball.y < 0 || ball.y > height as i32 {
            y_speed *= -1.0
        }

        bg.draw(&ctx)?;
        ball.draw(&ctx)?;

        Ok(())
    });

    Ok(())
}
