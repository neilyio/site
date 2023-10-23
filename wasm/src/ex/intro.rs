use crate::shapes::Drawer;
use crate::shapes::{self, Dot};
use crate::sketch::{Render, Sketch};
use anyhow::Result;
use noise::{NoiseFn, Perlin};
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use reqwest;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

pub struct Ex1 {
    dot: Dot,
    rng: ThreadRng,
}

impl Render for Ex1 {}
impl Sketch<CanvasRenderingContext2d> for Ex1 {
    async fn setup(ctx: &CanvasRenderingContext2d) -> Result<Self, anyhow::Error> {
        let width = ctx.canvas().unwrap().width();
        let height = ctx.canvas().unwrap().height();
        let rng = thread_rng();

        let dot = shapes::DotBuilder::default()
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

        Ok(Self { dot, rng })
    }

    async fn cycle(&mut self, ctx: &CanvasRenderingContext2d) -> Result<bool, anyhow::Error> {
        let d = 4; // Step size.

        for _ in 0..20 {
            // To step in any direction, including diagonal:
            self.dot.x += self.rng.gen_range(-1..2) * d;
            self.dot.y += self.rng.gen_range(-1..2) * d;
            self.dot.draw(&ctx).unwrap();
        }

        Ok(true)
    }
}

pub struct Ex2 {
    numbers: [i32; 20],
    rnd: ThreadRng,
    width: u32,
    height: u32,
}

impl Render for Ex2 {}
impl Sketch<CanvasRenderingContext2d> for Ex2 {
    async fn setup(ctx: &CanvasRenderingContext2d) -> Result<Self, anyhow::Error> {
        let numbers = [0; 20];
        let rnd = thread_rng();
        let width = ctx.canvas().unwrap().width();
        let height = ctx.canvas().unwrap().height();

        let bg = shapes::Background {
            color: "honeydew".to_string(),
        };
        bg.draw(&ctx).unwrap();

        Ok(Self {
            numbers,
            rnd,
            width,
            height,
        })
    }

    async fn cycle(&mut self, ctx: &CanvasRenderingContext2d) -> Result<bool, anyhow::Error> {
        // Increment a random number in the list.
        let index = self.rnd.gen_range(0..self.numbers.len());
        self.numbers[index] += 1;

        let bar_width = self.width / self.numbers.len() as u32;

        for (i, n) in self.numbers.iter().enumerate() {
            let rect = shapes::RectangleBuilder::default()
                .x(i as i32 * bar_width as i32)
                .y(self.height as i32)
                .width(bar_width as i32)
                .height(*n * -1)
                .color("red")
                .border_color("black")
                .border_width(2)
                .build()
                .unwrap();

            rect.draw(&ctx).unwrap();
        }

        Ok(true)
    }
}

pub struct Ex3 {
    dot: shapes::Dot,
    rng: ThreadRng,
    dist: WeightedIndex<usize>,
}

impl Render for Ex3 {}
impl Sketch<CanvasRenderingContext2d> for Ex3 {
    async fn setup(ctx: &CanvasRenderingContext2d) -> Result<Self, anyhow::Error> {
        let width = ctx.canvas().unwrap().width();
        let height = ctx.canvas().unwrap().height();

        let dot = shapes::DotBuilder::default()
            .x(width as i32 / 2)
            .y(height as i32 / 2)
            .color("red")
            .radius(1)
            .build()
            .unwrap();

        let rng = thread_rng();
        let dist = WeightedIndex::new([40, 20, 20, 20]).unwrap();

        let bg = shapes::Background {
            color: "honeydew".to_string(),
        };
        bg.draw(&ctx).unwrap();

        Ok(Self { dot, rng, dist })
    }

    async fn cycle(&mut self, ctx: &CanvasRenderingContext2d) -> Result<bool, anyhow::Error> {
        for _ in 0..10 {
            match self.dist.sample(&mut self.rng) {
                0 => self.dot.x += 1,
                2 => self.dot.x -= 1,
                3 => self.dot.y += 1,
                _ => self.dot.y -= 1,
            }

            self.dot.draw(&ctx).unwrap();
        }

        Ok(true)
    }
}

pub struct Ex4 {
    rng: ThreadRng,
    width: u32,
    height: u32,
}

impl Render for Ex4 {}
impl Sketch<CanvasRenderingContext2d> for Ex4 {
    async fn setup(ctx: &CanvasRenderingContext2d) -> Result<Self, anyhow::Error> {
        let width = ctx.canvas().unwrap().width();
        let height = ctx.canvas().unwrap().height();

        let rng = rand::thread_rng();

        let bg = shapes::Background {
            color: "honeydew".to_string(),
        };
        bg.draw(&ctx).unwrap();

        Ok(Self { rng, width, height })
    }

    async fn cycle(&mut self, ctx: &CanvasRenderingContext2d) -> Result<bool, anyhow::Error> {
        let num: f64 = self.rng.sample(StandardNormal);
        let sd = 60.0;
        let mean = self.width as f64 / 2.0;
        let x = sd * num + mean;

        let dot = shapes::DotBuilder::default()
            .x(x as i32)
            .y(self.height as i32 / 2)
            .color("red")
            .radius(10)
            .opacity(0.05)
            .build()
            .unwrap();

        dot.draw(&ctx).unwrap();

        Ok(true)
    }
}

pub struct Ex5 {
    perlin: Perlin,
    offset: f64,
    x: f64,
    y: f64,
}

impl Render for Ex5 {}
impl Sketch<CanvasRenderingContext2d> for Ex5 {
    async fn setup(ctx: &CanvasRenderingContext2d) -> Result<Self, anyhow::Error> {
        let width = ctx.canvas().unwrap().width();
        let height = ctx.canvas().unwrap().height();
        let perlin = Perlin::new(0);
        let offset = 0.0;
        let x = (width / 2) as f64;
        let y = (height / 2) as f64;

        let bg = shapes::Background {
            color: "honeydew".to_string(),
        };
        bg.draw(&ctx).unwrap();

        Ok(Self {
            perlin,
            offset,
            x,
            y,
        })
    }

    async fn cycle(&mut self, ctx: &CanvasRenderingContext2d) -> Result<bool, anyhow::Error> {
        let bg = shapes::Background {
            color: "honeydew".to_string(),
        };
        bg.draw(&ctx).unwrap();

        let scale = 0.02;
        let amplitude = 10.0;
        self.x = self.x + self.perlin.get([self.offset * scale, 0.0]) * amplitude;
        self.y = self.y + self.perlin.get([0.0, self.offset * scale]) * amplitude;

        let dot = shapes::DotBuilder::default()
            .x(self.x as i32)
            .y(self.y as i32)
            .color("red")
            .radius(10)
            .build()
            .unwrap();

        dot.draw(&ctx).unwrap();

        self.offset += 1.0;
        Ok(true)
    }
}

pub struct Ex6 {}

impl Render for Ex6 {}
impl Sketch<CanvasRenderingContext2d> for Ex6 {
    async fn setup(ctx: &CanvasRenderingContext2d) -> Result<Self, anyhow::Error> {
        let width = ctx.canvas().unwrap().width() as usize;
        let height = ctx.canvas().unwrap().height() as usize;

        // Fetching data from /perlin2d
        let url = format!(
            "http://localhost:3000/api/perlin2d?width={}&height={}",
            width, height
        );
        let response = reqwest::get(&url).await.unwrap();
        let data: Vec<u8> = response.bytes().await.unwrap().to_vec();

        // Put the modified ImageData back onto the canvas
        let new_image = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            width as u32,
            height as u32,
        )
        .unwrap();
        ctx.put_image_data(&new_image, 0.0, 0.0).unwrap();

        Ok(Self {})
    }
}
