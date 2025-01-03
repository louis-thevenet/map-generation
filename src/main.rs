use image::Rgb;
use noise_to_img::{Layer, NoiseToImage};
use perlin_gen::PerlinNoiseGenerator;
mod noise_to_img;
mod perlin_gen;

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn main() {
    const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
    const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
    const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
    const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);
    let dimension = 1024 * 10;
    let perlin_gen = PerlinNoiseGenerator::new(dimension as usize).add_octaves(1);
    let image_gen = NoiseToImage::new(dimension)
        .add_layer(Layer {
            treshold: 0.85,
            color: WHITE,
        })
        .add_layer(Layer {
            treshold: 0.5,
            color: GREEN,
        })
        .add_layer(Layer {
            treshold: 0.48,
            color: YELLOW,
        })
        .add_layer(Layer {
            treshold: 0.0,
            color: BLUE,
        });

    let _ = image_gen.create_image(&perlin_gen).save("output.png");
}
