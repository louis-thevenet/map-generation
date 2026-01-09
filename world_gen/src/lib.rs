use biome::BiomeSettings;
use cell::Cell;
use perlin_noise::PerlinNoiseGenerator;
use rand::{thread_rng, RngCore};

mod biome;
pub mod cell;
pub mod city_generation;
pub mod image_utils;
mod perlin_noise;
mod vector;
/// So that 1.0 is a good scale
const GLOBAL_SCALE_FIX: f64 = 30.;
#[derive(Debug, Clone)]

pub struct WorldGen {
    pub seed: u64,
    temperature_noise: PerlinNoiseGenerator,
    moisture_noise: PerlinNoiseGenerator,
    continentalness_noise: PerlinNoiseGenerator,
    erosion_noise: PerlinNoiseGenerator,
    temp_scale: f64,
    moisture_scale: f64,
    continentalness_scale: f64,
    erosion_scale: f64,
}

impl Default for WorldGen {
    fn default() -> Self {
        Self::new(1.0, None)
    }
}
impl WorldGen {
    #[must_use]
    pub fn new(global_scale: f64, seed_opt: Option<u64>) -> Self {
        let seed = if let Some(seed_value) = seed_opt {
            seed_value
        } else {
            let seed_value = thread_rng().next_u64();
            println!("Seed: {seed_value}");
            seed_value
        };

        let temp_scale = GLOBAL_SCALE_FIX * global_scale * 64.;
        let moisture_scale = GLOBAL_SCALE_FIX * global_scale * 64.;
        let continentalness_scale = GLOBAL_SCALE_FIX * global_scale * 64.;
        let erosion_scale = GLOBAL_SCALE_FIX * global_scale * 16.;
        Self {
            seed,
            temperature_noise: PerlinNoiseGenerator::new(seed)
                .set_lacunarity(1.3)
                .set_persistence(0.5)
                .set_octaves(4),
            temp_scale,
            moisture_noise: PerlinNoiseGenerator::new(seed + 2)
                .set_lacunarity(1.3)
                .set_persistence(0.5)
                .set_octaves(4),
            moisture_scale,
            continentalness_noise: PerlinNoiseGenerator::new(seed + 4)
                .set_lacunarity(1.7)
                .set_persistence(0.6)
                .set_octaves(8),
            continentalness_scale,

            erosion_noise: PerlinNoiseGenerator::new(seed + 8)
                .set_lacunarity(2.0)
                .set_persistence(0.5)
                .set_octaves(8),
            erosion_scale,
        }
    }
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn generate_cell(&self, pos: (isize, isize)) -> Cell {
        let pos = (pos.0 as f64, pos.1 as f64);
        let temp = self.temperature_noise.noise(pos, self.temp_scale);
        let moisture = self.moisture_noise.noise(pos, self.moisture_scale);
        let continentalness = self
            .continentalness_noise
            .noise(pos, self.continentalness_scale);
        let erosion = self.erosion_noise.noise(pos, self.erosion_scale);

        Cell {
            temp,
            moisture,
            continentalness,
            erosion,
            biome: BiomeSettings::new(temp, moisture, continentalness, erosion).into(),
        }
    }
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    /// Generate a cell with a specific scale (for zooming in/out)
    pub fn generate_cell_scaled(&self, scale: f64, pos: (isize, isize)) -> Cell {
        let pos = (pos.0 as f64, pos.1 as f64);

        let temp = self.temperature_noise.noise(pos, self.temp_scale / scale);
        let moisture = self.moisture_noise.noise(pos, self.moisture_scale / scale);
        let continentalness = self
            .continentalness_noise
            .noise(pos, self.continentalness_scale / scale);
        let erosion = self.erosion_noise.noise(pos, self.erosion_scale / scale);

        Cell {
            temp,
            moisture,
            continentalness,
            erosion,
            biome: BiomeSettings::new(temp, moisture, continentalness, erosion).into(),
        }
    }

    pub fn generate_chunk(&self, position: [i32; 2], size: [u32; 2]) -> Vec<Vec<Cell>> {
        let mut chunk = Vec::with_capacity(size[1] as usize);
        for y in 0..size[1] {
            let mut row = Vec::with_capacity(size[0] as usize);
            for x in 0..size[0] {
                let cell = self.generate_cell((
                    position[0] as isize + x as isize,
                    position[1] as isize + y as isize,
                ));
                row.push(cell);
            }
            chunk.push(row);
        }
        chunk
    }
}
