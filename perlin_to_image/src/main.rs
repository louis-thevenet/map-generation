use game_core::terrain_generator::perlin_noise::PerlinNoiseGenerator;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

const PERMUTATION_LENGTH: usize = 1024;

use image::{ImageBuffer, Rgb};
use rayon::iter::ParallelIterator;

pub fn draw_grid(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, cell_size: u32) {
    const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
    img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
        if x % cell_size == 0 || y % cell_size == 0 {
            *p = BLACK;
        }
    });
}
pub fn draw_rect(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pos: (u32, u32),
    width: u32,
    height: u32,
    color: Rgb<u8>,
) {
    for x in 0..=width {
        *img.get_pixel_mut(pos.0 + x, pos.1) = color;
        *img.get_pixel_mut(pos.0 + x, pos.1 + height) = color;
    }
    for y in 0..=height {
        *img.get_pixel_mut(pos.0, pos.1 + y) = color;
        *img.get_pixel_mut(pos.0 + width, pos.1 + y) = color;
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#[must_use]
pub fn create_image(
    size: (u32, u32),
    noise_gen: &PerlinNoiseGenerator,
    scale: f64,
    permutations: &[usize],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(size.0, size.1);
    img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
        let pos = (f64::from(x), f64::from(y));

        let noise = (noise_gen.noise(pos, scale, permutations) + 1.) / 2.;

        // default pixel color is either lowest layer or noise map if layers is empty
        let px = Rgb([(noise * 255.0) as u8; 3]);

        *p = px;
    });
    img
}
fn main() -> Result<(), image::ImageError> {
    // Default terrain
    // let perlin_gen = PerlinNoiseGenerator::default()
    //     .set_octaves(8)
    //     .set_persistance(0.5)
    //     .set_lacunarity(2.0);
    // let scale = 16.0;

    // Default temeprature
    let perlin_gen = PerlinNoiseGenerator::default()
        .set_octaves(8)
        .set_persistance(0.4)
        .set_lacunarity(2.0);
    let scale = 64.0;

    let mut permutations: Vec<usize> = (0..=PERMUTATION_LENGTH).collect::<Vec<usize>>();
    let seed = 1;

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    permutations.shuffle(&mut rng);
    create_image((1000, 1000), &perlin_gen, scale, &permutations).save("output.png")
}
