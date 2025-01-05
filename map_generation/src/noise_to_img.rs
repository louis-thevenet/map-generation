use image::{ImageBuffer, Rgb};
use rayon::iter::ParallelIterator;
use terrain_generator::TerrainGenerator;

use crate::terrain_generator;

/// A color and a treshhold to color maps images based on noise.
pub struct Layer {
    pub treshold: f64,
    pub color: Rgb<u8>,
}
#[derive(Default)]
/// Settings for the image to create.
pub struct NoiseToImage {
    layers: Vec<Layer>,
}
impl NoiseToImage {
    /// Draws a grid on a image using cell length.
    pub fn draw_grid(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, cell_size: u32) {
        const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
        img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
            if x % cell_size == 0 || y % cell_size == 0 {
                *p = BLACK;
            }
        });
    }
    /// Draws a rectangle on an image from a top-left pixel position and width and height.
    /// Only the rectangle sides are colored.
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
    #[must_use]
    /// Adds a new layer to the `NoiseToImage`
    pub fn add_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self.layers
            .sort_by(|l1, l2| l2.treshold.total_cmp(&l1.treshold));
        self
    }
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    /// Converts a 2D noise vector (with values from -1.0 to 1.0) to an image.
    pub fn vec_to_image(&self, data: &[Vec<f64>]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(data[0].len() as u32, data.len() as u32);
        img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
            let noise = (1.0 + data[y as usize][x as usize]) / 2.0;

            // default pixel color is either lowest layer or noise map if layers is empty
            let mut px = self
                .layers
                .last()
                .unwrap_or(&Layer {
                    treshold: 0.0,
                    color: Rgb([(noise * 255.0) as u8; 3]),
                })
                .color;

            for layer in &self.layers {
                if noise >= layer.treshold {
                    px = layer.color;
                    break;
                }
            }
            *p = px;
        });
        img
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    /// Creates an image with specified `size` using the given `PerlinNoiseGenerator`.
    pub fn create_image(
        &self,
        size: (u32, u32),
        noise_gen: &TerrainGenerator,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(size.0, size.1);
        img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
            let pos = (f64::from(x), f64::from(y));

            let noise = (noise_gen.noise(pos) + 1.) / 2.;

            // default pixel color is either lowest layer or noise map if layers is empty
            let mut px = self
                .layers
                .last()
                .unwrap_or(&Layer {
                    treshold: 0.0,
                    color: Rgb([(noise * 255.0) as u8; 3]),
                })
                .color;

            for layer in &self.layers {
                if noise >= layer.treshold {
                    px = layer.color;
                    break;
                }
            }
            *p = px;
        });
        img
    }
}
