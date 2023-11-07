use crate::utils::animation_frame;
use wasm_bindgen_futures::spawn_local;
use web_sys::CanvasRenderingContext2d;

pub trait Sketch<T> {
    async fn setup(ctx: &T) -> Result<Self, anyhow::Error>
    where
        Self: Sized;

    async fn cycle(&mut self, _: &T) -> Result<bool, anyhow::Error> {
        Ok(false)
    }
}

pub trait Render: Sketch<WebCanvas> {
    fn render(ctx: WebCanvas) -> Result<(), anyhow::Error>
    where
        Self: Sized,
    {
        spawn_local(async move {
            let mut ex = Self::setup(&ctx).await.unwrap();

            loop {
                let keep_going = ex.cycle(&ctx).await.unwrap();
                if !keep_going {
                    break;
                }

                let promise = wasm_bindgen_futures::JsFuture::from(animation_frame());
                promise.await.unwrap();
            }
        });

        Ok(())
    }
}

pub struct WebCanvas {
    ctx: CanvasRenderingContext2d,
}

impl WebCanvas {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx }
    }

    pub fn width(&self) -> u32 {
        self.canvas().width()
    }

    pub fn height(&self) -> u32 {
        self.canvas().height()
    }

    pub fn wh(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn canvas(&self) -> web_sys::HtmlCanvasElement {
        self.ctx.canvas().unwrap()
    }

    pub fn ctx(&self) -> &CanvasRenderingContext2d {
        &self.ctx
    }
}
