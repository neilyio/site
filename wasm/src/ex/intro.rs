use crate::shapes;
use crate::shapes::Drawer;
use crate::utils;
use js_sys::Object;
use nalgebra::Vector4;
use noise::{NoiseFn, Perlin};
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use std::{cell::RefCell, rc::Rc};
use utils::canvas_context;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub fn ex1(ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
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

pub fn ex2(ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
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

pub fn ex3(ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
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

    bg.draw(&ctx)?;
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

pub fn ex4(ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();

    let bg = shapes::Background {
        color: "honeydew".to_string(),
    };

    let mut rng = rand::thread_rng();

    bg.draw(&ctx)?;

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

pub fn ex5(ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
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

pub fn ex6(ctx: CanvasRenderingContext2d) -> Result<(), JsValue> {
    let width = ctx.canvas().unwrap().width();
    let height = ctx.canvas().unwrap().height();
    let perlin = Perlin::new(0);

    // Retrieve the existing ImageData or create a new one
    let image_data = ctx
        .create_image_data_with_sw_and_sh(width as f64, height as f64)
        .unwrap();

    let mut data = image_data.data();

    let foreground = Vector4::new(255.0, 0.0, 0.0, 255.0);
    let background = Vector4::new(240.0, 255.0, 240.0, 255.0);
    let octaves: usize = 6;

    // Loop through the pixels and adjust their colors based on Perlin noise
    for x in 0..width {
        for y in 0..height {
            let mut noise_val = 0.0;
            let mut scale = 0.02;
            let mut amplitude = 1.0;
            for _ in 0..octaves {
                noise_val += perlin.get([x as f64 * scale, y as f64 * scale]) * amplitude;
                scale *= 2.0; // Double the frequency for the next octave
                amplitude *= 0.5; // Halve the amplitude for the next octave
            }

            let normalized = (noise_val + 1.0) / 2.0;

            let color = foreground.lerp(&background, normalized);
            let index = (4 * (y * width + x)) as usize;

            data[index] = color.x as u8; // R
            data[index + 1] = color.y as u8; // G
            data[index + 2] = color.z as u8; // B
            data[index + 3] = color.w as u8; // A (fully opaque)
        }
    }

    // Put the modified ImageData back onto the canvas
    let new_image = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    ctx.put_image_data(&new_image, 0.0, 0.0).unwrap();

    Ok(())
}
