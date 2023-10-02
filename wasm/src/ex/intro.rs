use crate::shapes;
use crate::shapes::Drawer;
use crate::utils;
use anyhow::Result;
use js_sys::Object;
use nalgebra::Vector4;
use noise::{NoiseFn, Perlin};
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use reqwest;
use shared::noise::perlin_2d_array;
use std::{cell::RefCell, rc::Rc};
use utils::canvas_context;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub async fn ex1(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();
    let mut rng = thread_rng();

    let mut w = shapes::DotBuilder::default()
        .x(width as i32 / 2)
        .y(height as i32 / 2)
        .radius(1)
        .color("red")
        .build()
        .unwrap();

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    bg.draw(&ctx).unwrap();
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

pub async fn ex2(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().width();
    let mut numbers: [i32; 20] = [0; 20];
    let mut rnd = thread_rng();

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    bg.draw(&ctx).unwrap();
    utils::raf_loop(move || {
        // Increment a random number in the list.
        let index = rnd.gen_range(0..numbers.len());
        numbers[index] += 1;

        let bar_width = width / numbers.len() as u32;

        for (i, n) in numbers.iter().enumerate() {
            let rect = shapes::RectangleBuilder::default()
                .x(i as i32 * bar_width as i32)
                .y(height as i32)
                .width(bar_width as i32)
                .height(*n * -1)
                .color("red")
                .border_color("black")
                .border_width(2)
                .build()
                .unwrap();

            rect.draw(&ctx)?;
        }

        Ok(())
    });
    Ok(())
}

pub async fn ex3(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();

    let mut w = shapes::DotBuilder::default()
        .x(width as i32 / 2)
        .y(height as i32 / 2)
        .color("red")
        .radius(1)
        .build()
        .unwrap();

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    let mut rng = thread_rng();
    let dist = WeightedIndex::new([40, 20, 20, 20]).unwrap();

    bg.draw(&ctx).unwrap();
    utils::raf_loop(move || {
        for _ in 0..10 {
            // To step in any direction, including diagonal:
            // w.x += rng.gen_range(-1..2) * d;
            // w.y += rng.gen_range(-1..2) * d;

            match dist.sample(&mut rng) {
                0 => w.x += 1,
                2 => w.x -= 1,
                3 => w.y += 1,
                _ => w.y -= 1,
            }

            w.draw(&ctx)?;
        }

        Ok(())
    });

    Ok(())
}

pub async fn ex4(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    let mut rng = rand::thread_rng();

    bg.draw(&ctx).unwrap();

    utils::raf_loop(move || {
        let num: f64 = rng.sample(StandardNormal);
        let sd = 60.0;
        let mean = width as f64 / 2.0;
        let x = sd * num + mean;

        let dot = shapes::DotBuilder::default()
            .x(x as i32)
            .y(height as i32 / 2)
            .color("red")
            .radius(10)
            .opacity(0.05)
            .build()
            .unwrap();

        dot.draw(&ctx)?;

        Ok(())
    });

    Ok(())
}

pub async fn ex5(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();
    let perlin = Perlin::new(0);

    let mut offset = 0.0;
    let mut x = (width / 2) as f64;
    let mut y = (height / 2) as f64;

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    utils::raf_loop(move || {
        bg.draw(&ctx)?;

        let scale = 0.02;
        let amplitude = 10.0;
        x = x + perlin.get([offset * scale, 0.0]) * amplitude;
        y = y + perlin.get([0.0, offset * scale]) * amplitude;

        let dot = shapes::DotBuilder::default()
            .x(x as i32)
            .y(y as i32)
            .color("red")
            .radius(10)
            .build()
            .unwrap();

        dot.draw(&ctx)?;

        offset += 1.0;
        Ok(())
    });

    Ok(())
}

pub async fn ex6(ctx: CanvasRenderingContext2d) -> Result<()> {
    let width = ctx.canvas().unwrap().width() as usize;
    let height = ctx.canvas().unwrap().height() as usize;

    // Fetching data from /perlin2d
    let url = format!(
        "http://localhost:3000/api/perlin2d?width={}&height={}",
        width, height
    );
    let response = reqwest::get(&url).await?;
    let data: Vec<u8> = response.bytes().await?.to_vec();

    // Put the modified ImageData back onto the canvas
    let new_image =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width as u32, height as u32)
            .unwrap();
    ctx.put_image_data(&new_image, 0.0, 0.0).unwrap();

    Ok(())
}
