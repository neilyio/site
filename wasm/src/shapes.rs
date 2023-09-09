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
    pub opacity: f64,
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

        ctx.set_global_alpha(self.opacity);
        // Fill the circle
        ctx.fill();

        ctx.set_global_alpha(1.0);

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

pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub color: String,
    pub border_color: String,
}

impl Drawer for Rectangle {
    fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
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
