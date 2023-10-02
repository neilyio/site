pub use nalgebra::Vector4;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::ops::IndexMut;

pub fn perlin_2d_array<T>(
    data: &mut T,
    width: usize,
    height: usize,
    octaves: usize,
    foreground: Vector4<f64>,
    background: Vector4<f64>,
) where
    T: IndexMut<usize, Output = u8>,
{
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    let perlin = Perlin::new(seed);
    // let mut data = vec![0u8; 4 * width * height];

    for x in 0..width {
        for y in 0..height {
            let mut noise_val = 0.0;
            let mut scale = 0.02;
            let mut amplitude = 1.0;
            for _ in 0..octaves {
                noise_val += perlin.get([x as f64 * scale, y as f64 * scale]) * amplitude;
                scale *= 2.0;
                amplitude *= 0.5;
            }

            let normalized = (noise_val + 1.0) / 2.0;
            let color = foreground.lerp(&background, normalized);
            let index = (4 * (y * width + x)) as usize;

            data[index] = color.x as u8;
            data[index + 1] = color.y as u8;
            data[index + 2] = color.z as u8;
            data[index + 3] = color.w as u8;
        }
    }
}
