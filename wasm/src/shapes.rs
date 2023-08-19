use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub trait Drawer {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>;
}

pub struct Dot {
    pub x: i32,
    pub y: i32,
    pub color: String,
    pub radius: i32,
}

impl Drawer for Dot {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
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

        // Fill the circle
        ctx.fill();

        Ok(())
    }
}

pub struct Background {
    pub color: String,
}

impl Drawer for Background {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let width = ctx.canvas().unwrap().width() as f64;
        let height = ctx.canvas().unwrap().height() as f64;

        // Draw a rectangle
        ctx.set_fill_style(&self.color.clone().into());
        ctx.fill_rect(0.0, 0.0, width, height);

        Ok(())
    }
}
