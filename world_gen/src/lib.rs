use biome::BiomeSettings;
use cell::Cell;
use perlin_noise::PerlinNoiseGenerator;
use rand::{thread_rng, RngCore};

mod biome;
pub mod cell;
mod perlin_noise;
mod vector;

#[derive(Debug, Clone)]

pub struct WorldGen {
    pub chunk_size: isize,
    temperature_noise: PerlinNoiseGenerator,
    moisture_noise: PerlinNoiseGenerator,
    continentalness_noise: PerlinNoiseGenerator,
    erosion_noise: PerlinNoiseGenerator,
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

        let temp_scale = global_scale * 64.;
        let moisture_scale = global_scale * 64.;
        let continentalness_scale = global_scale * 64.;
        let erosion_scale = global_scale * 16.;
        Self {
            chunk_size: 32,
            temperature_noise: PerlinNoiseGenerator::new(seed)
                .set_scale(temp_scale)
                .set_lacunarity(1.3)
                .set_persistance(0.5)
                .set_octaves(4),
            moisture_noise: PerlinNoiseGenerator::new(seed + 2)
                .set_scale(moisture_scale)
                .set_lacunarity(1.3)
                .set_persistance(0.5)
                .set_octaves(4),
            continentalness_noise: PerlinNoiseGenerator::new(seed + 4)
                .set_scale(continentalness_scale)
                .set_lacunarity(1.7)
                .set_persistance(0.6)
                .set_octaves(8),
            erosion_noise: PerlinNoiseGenerator::new(seed + 8)
                .set_scale(erosion_scale)
                .set_lacunarity(2.0)
                .set_persistance(0.5)
                .set_octaves(8),
        }
    }
    pub fn update_scale(&mut self, global_scale: f64) {
        self.temperature_noise = self.temperature_noise.set_scale(global_scale * 64.);
        self.moisture_noise = self.moisture_noise.set_scale(global_scale * 64.);
        self.continentalness_noise = self.continentalness_noise.set_scale(global_scale * 64.);
        self.erosion_noise = self.erosion_noise.set_scale(global_scale * 16.);
    }

    /// Generates a chunk from its coordinates.
    /// Chunk dimension are
    /// (pos.0, pos.1)                 ...   (pos.0 + `chunk_size`, pos.1)
    ///                                ...
    /// (pos.0, pos.1 + `chunk_size`)  ...   (pos.0 + `chunk_size`, pos.1 `chunk_size`ze)
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn generate_chunk(&self, pos: (isize, isize)) -> Cell {
        let pos = (pos.0 as f64, pos.1 as f64);

        let temp = self.temperature_noise.noise(pos);
        let moisture = self.moisture_noise.noise(pos);
        let continentalness = self.continentalness_noise.noise(pos);
        let erosion = self.erosion_noise.noise(pos);

        Cell {
            temp,
            moisture,
            continentalness,
            erosion,
            biome: BiomeSettings::new(temp, moisture, continentalness, erosion).into(),
        }
    }
}
