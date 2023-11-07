use crate::{
    shapes::{self, Drawer},
    sketch::{Render, Sketch, WebCanvas},
};

pub struct Ex1 {
    width: usize,
    height: usize,
    xspeed: f32,
    yspeed: f32,
    ball: shapes::Dot,
    background: shapes::Background,
}

impl Render for Ex1 {}
impl Sketch<WebCanvas> for Ex1 {
    async fn setup(ctx: &WebCanvas) -> std::result::Result<Self, anyhow::Error> {
        let (width, height) = ctx.wh();

        let ball = shapes::Dot {
            x: width as i32 / 2,
            y: height as i32 / 2,
            radius: 32,
            color: "red".to_string(),
            opacity: 1.0,
        };

        let background = shapes::Background {
            color: "honeydew".to_string(),
        };

        let xspeed = 10.0;
        let yspeed = 15.0;

        background.draw(&ctx).unwrap();
        ball.draw(&ctx).unwrap();

        Ok(Self {
            width: width as usize,
            height: height as usize,
            xspeed,
            yspeed,
            ball,
            background,
        })
    }

    async fn cycle(&mut self, ctx: &WebCanvas) -> std::result::Result<bool, anyhow::Error> {
        self.ball.x += self.xspeed as i32;
        self.ball.y += self.yspeed as i32;

        if self.ball.x < 0 || self.ball.x > self.width as i32 {
            self.xspeed *= -1.0
        }

        if self.ball.y < 0 || self.ball.y > self.height as i32 {
            self.yspeed *= -1.0
        }

        self.background.draw(&ctx).unwrap();
        self.ball.draw(&ctx).unwrap();

        Ok(true)
    }
}
