#![allow(dead_code, unused_variables, unused_imports)]
// pub mod canvas;
// pub mod shapes;

pub use crate::renderer::canvas::CanvasRenderer;
use crate::renderer::shapes::Dot;
use js_sys;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub trait Drawable<'a, T: Renderer<'a>> {
    type Result;

    fn draw(&self, renderer: &T) -> Self::Result;
}

pub mod shapes {
    use derive_builder::Builder;
    use wasm_bindgen::JsValue;
    use web_sys::CanvasRenderingContext2d;

    pub trait Drawer {
        fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>;
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

    #[derive(Builder)]
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
}

pub mod canvas {
    use crate::renderer::shapes::*;
    use crate::renderer::{Drawable, Renderer};
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use web_sys::{window, CanvasRenderingContext2d};

    pub struct CanvasRenderer {
        pub ctx: Rc<CanvasRenderingContext2d>,
        pub window: web_sys::Window,
    }

    impl CanvasRenderer {
        pub fn new(window: web_sys::Window, ctx: CanvasRenderingContext2d) -> Self {
            Self {
                window,
                ctx: Rc::new(ctx),
            }
        }

        fn request_animation_frame(&self, f: &Closure<dyn FnMut()>) {
            self.window
                .request_animation_frame(f.as_ref().unchecked_ref())
                .expect("should register `requestAnimationFrame` OK");
        }
    }

    impl<'a> Renderer<'a> for CanvasRenderer {
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

        // fn cycle(&self, mut callback: impl FnMut() -> Result<(), JsValue> + 'a) {
        //     let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        //     let g = f.clone();

        //     let closure = move || {
        //         callback().unwrap();

        //         // Schedule ourself for another requestAnimationFrame callback.
        //         self.request_animation_frame(
        //             f.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref(),
        //         );
        //     };

        //     let boxed_closure: Box<dyn FnMut()> = Box::new(closure);
        //     let js_closure = Closure::wrap(boxed_closure).into_jsvalue();

        //     *g.borrow_mut() = Some(Closure::wrap(boxed_closure));

        //     self.request_animation_frame(
        //         g.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref(),
        //     );
        // }
        fn cycle(&self, mut callback: impl FnMut() -> Result<(), JsValue> + 'a) {
            let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
            let g = f.clone();

            let closure = move || {
                callback().unwrap();
                // Schedule ourself for another requestAnimationFrame callback.
                self.request_animation_frame(f.borrow().as_ref().unwrap_throw());
            };

            *g.borrow_mut() = Some(Closure::wrap(Box::new(closure)));

            self.request_animation_frame(g.borrow().as_ref().unwrap_throw());
        }
    }

    impl<'a> Drawable<'a, CanvasRenderer> for Dot {
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
    impl<'a> Drawable<'a, CanvasRenderer> for Background {
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
    impl<'a> Drawable<'a, CanvasRenderer> for Rectangle {
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
}

// pub trait Drawable<T>
// where
//     T: Renderer,
// {
//     type Result;

//     fn draw(&self, renderer: &T) -> Self::Result;
// }

pub trait Renderer<'a> {
    type Result: core::fmt::Debug;

    fn draw(&self, drawable: &dyn Drawable<Self, Result = Self::Result>) -> Self::Result;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn cycle(&self, callback: impl FnMut() -> Result<(), JsValue> + 'a);
}
