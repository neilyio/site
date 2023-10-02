use crate::renderer::shapes::*;
use crate::renderer::{Drawable, Renderer};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct CanvasRenderer {
    pub ctx: Rc<CanvasRenderingContext2d>,
}

impl CanvasRenderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx: Rc::new(ctx) }
    }
}

impl Renderer for CanvasRenderer {
    type Result = Result<(), JsValue>;

    fn draw(&self, drawable: &dyn Drawable<Self, Result = Self::Result>) -> Self::Result {
        drawable.draw(&self)
    }

    fn width(&self) -> u32 {
        self.ctx.canvas().unwrap().width()
    }

    fn height(&self) -> u32 {
        self.ctx.canvas().unwrap().height()
    }
}

impl Drawable<CanvasRenderer> for Dot {
    type Result = Result<(), JsValue>;

    fn draw(&self, renderer: &CanvasRenderer) -> Self::Result {
        let ctx = &renderer.ctx;
        // Set the fill color
        ctx.set_fill_style(&self.color.clone().into());

        // Start the path
        ctx.begin_path();

        // Draw a circle
        ctx.arc(
            self.x as f64,
            self.y as f64,
            self.radius as f64,
            0.0,
            2.0 * std::f64::consts::PI,
        )?;

        ctx.set_global_alpha(self.opacity);
        // Fill the circle
        ctx.fill();

        ctx.set_global_alpha(1.0);

        Ok(())
    }
}
impl Drawable<CanvasRenderer> for Background {
    type Result = Result<(), JsValue>;

    fn draw(&self, renderer: &CanvasRenderer) -> Self::Result {
        let ctx = &renderer.ctx;
        let width = ctx.canvas().unwrap().width() as f64;
        let height = ctx.canvas().unwrap().height() as f64;

        // Draw a rectangle
        ctx.set_fill_style(&self.color.clone().into());
        ctx.fill_rect(0.0, 0.0, width, height);

        Ok(())
    }
}
impl Drawable<CanvasRenderer> for Rectangle {
    type Result = Result<(), JsValue>;

    fn draw(&self, renderer: &CanvasRenderer) -> Self::Result {
        let ctx = &renderer.ctx;

        ctx.set_fill_style(&self.color.clone().into());
        ctx.fill_rect(
            self.x as f64,
            self.y as f64,
            self.width as f64,
            self.height as f64,
        );

        ctx.set_stroke_style(&self.border_color.clone().into()); // Set the border color
        ctx.set_line_width(self.border_width as f64); // Set the border thickness
        ctx.stroke_rect(
            (self.x + (self.border_width / 2)) as f64,
            (self.y + (self.border_width / 2)) as f64,
            (self.width - (self.border_width / 2)) as f64,
            (self.height - (self.border_width / 2)) as f64,
        );

        Ok(())
    }
}
