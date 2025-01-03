use image::{ImageBuffer, Rgb};
use rayon::iter::ParallelIterator;

use crate::perlin_gen::PerlinNoiseGenerator;

pub struct Layer {
    pub treshold: f64,
    pub color: Rgb<u8>,
}
pub struct NoiseToImage {
    dimension: u32,
    layers: Vec<Layer>,
}
impl NoiseToImage {
    pub fn new(dimension: u32) -> Self {
        Self {
            dimension,
            layers: vec![],
        }
    }

    pub fn add_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn create_image(
        mut self,
        scale: f64,
        noise_gen: &PerlinNoiseGenerator,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        self.layers
            .sort_by(|l1, l2| l2.treshold.total_cmp(&l1.treshold));

        let mut img = ImageBuffer::new(self.dimension, self.dimension);
        img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
            let pos = (f64::from(x) / scale, f64::from(y) / scale);

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
