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
    let dimension = 1000;
    let perlin_gen = PerlinNoiseGenerator::new(dimension as usize, Some(9_776_883_071_826_648_804))
        .set_lacunarity(2.0)
        .set_persistance(0.5)
        .set_octaves(8);
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
            treshold: 0.45,
            color: YELLOW,
        })
        .add_layer(Layer {
            treshold: 0.0,
            color: BLUE,
        });

    let mut create_image = image_gen.create_image(40.0, &perlin_gen);
    let px = create_image.get_pixel_mut(dimension / 2, dimension / 2);
    *px = Rgb([255, 0, 0]);
    let _ = create_image.save("output.png");
}
