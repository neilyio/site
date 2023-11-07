use crate::sketch::WebCanvas;
use derive_builder::Builder;
use wasm_bindgen::JsValue;

pub trait Drawer<T> {
    type Result;

    fn draw(&self, ctx: T) -> Self::Result;
}

#[derive(Builder, Default)]
pub struct Dot {
    pub x: i32,
    pub y: i32,
    #[builder(setter(into))]
    pub color: String,
    pub radius: i32,
    #[builder(default = "1.0")]
    pub opacity: f64,
}

impl Drawer<&WebCanvas> for Dot {
    type Result = Result<(), JsValue>;

    fn draw(&self, ctx: &WebCanvas) -> Self::Result {
        let ctx = ctx.ctx();
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

#[derive(Builder)]
pub struct Background {
    pub color: String,
}

impl Drawer<&WebCanvas> for Background {
    type Result = Result<(), JsValue>;

    fn draw(&self, ctx: &WebCanvas) -> Self::Result {
        let (width, height) = ctx.wh();
        let ctx = ctx.ctx();

        // Draw a rectangle
        ctx.set_fill_style(&self.color.clone().into());
        ctx.fill_rect(0.0, 0.0, width as f64, height as f64);

        Ok(())
    }
}

#[derive(Builder, Default)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    #[builder(setter(into))]
    pub color: String,
    #[builder(setter(into))]
    pub border_color: String,
}

impl Drawer<&WebCanvas> for Rectangle {
    type Result = Result<(), JsValue>;

    fn draw(&self, ctx: &WebCanvas) -> Self::Result {
        let ctx = ctx.ctx();
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
