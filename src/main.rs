use image::{ImageBuffer, Rgb};
use perlin_gen::PerlinNoiseGenerator;
use rayon::iter::ParallelIterator;
mod perlin_gen;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn main() {
    const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
    const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
    const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
    const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);
    let dimension = 1024 * 8;
    let perlin_gen = PerlinNoiseGenerator::new(dimension as usize);
    let mut img = ImageBuffer::new(dimension, dimension);
    img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
        let pos = (f64::from(x), f64::from(y));

        let noise = (perlin_gen.fractal_brownian_motion(pos, 8) + 1.) / 2.;

        let px = if noise > 0.85 {
            WHITE
        } else if noise > 0.5 {
            GREEN
        } else if noise > 0.48 {
            YELLOW
        } else {
            BLUE
        };

        *p = px;
    });
    let _ = img.save("output.png");
}
