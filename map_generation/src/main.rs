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
    const RED: Rgb<u8> = Rgb([255, 0, 0]);
    let perlin_gen = PerlinNoiseGenerator::new(64, Some(4))
        .set_lacunarity(2.0)
        .set_persistance(0.5)
        .set_octaves(8)
        .set_scale(40.0);
    let image_gen = NoiseToImage::default()
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

    let chunk_to_draw = (2, 4);

    // Draw chunks
    let dimension = 64 * perlin_gen.chunk_size as u32;
    let mut chunks_img = image_gen.create_image((dimension, dimension), &perlin_gen);
    NoiseToImage::draw_grid(&mut chunks_img, perlin_gen.chunk_size as u32);
    NoiseToImage::draw_rect(
        &mut chunks_img,
        (
            (chunk_to_draw.0 * perlin_gen.chunk_size) as u32,
            (chunk_to_draw.1 * perlin_gen.chunk_size) as u32,
        ),
        perlin_gen.chunk_size as u32,
        perlin_gen.chunk_size as u32,
        RED,
    );
    let _ = chunks_img.save("output.png");

    // Draw one chunk
    let single_chunk_img = image_gen.vec_to_image(&perlin_gen.generate_chunk(chunk_to_draw));
    let _ = single_chunk_img.save("chunk_0_0.png");
}
