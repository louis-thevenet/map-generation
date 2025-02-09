use chunk::Chunk;
use perlin_noise::PerlinNoiseGenerator;
use rand::{thread_rng, RngCore};

mod biome;
pub mod chunk;
mod perlin_noise;
mod vector;

#[derive(Debug)]

pub struct WorldGen {
    pub chunk_size: isize,
    temperature_noise: PerlinNoiseGenerator,
    moisture_noise: PerlinNoiseGenerator,
    continentalness_noise: PerlinNoiseGenerator,
    erosion_noise: PerlinNoiseGenerator,
}

impl Default for WorldGen {
    fn default() -> Self {
        Self::new(None)
    }
}

impl WorldGen {
    #[must_use]
    pub fn new(seed_opt: Option<u64>) -> Self {
        let seed = if let Some(seed_value) = seed_opt {
            seed_value
        } else {
            let seed_value = thread_rng().next_u64();
            println!("Seed: {seed_value}");
            seed_value
        };
        Self {
            chunk_size: 32,
            temperature_noise: PerlinNoiseGenerator::new(seed)
                .set_scale(64.)
                .set_lacunarity(1.3)
                .set_persistance(0.5)
                .set_octaves(4),
            moisture_noise: PerlinNoiseGenerator::new(seed + 2)
                .set_scale(64.)
                .set_lacunarity(1.3)
                .set_persistance(0.5)
                .set_octaves(4),
            continentalness_noise: PerlinNoiseGenerator::new(seed + 4)
                .set_scale(16.)
                .set_lacunarity(1.7)
                .set_persistance(0.6)
                .set_octaves(8),
            erosion_noise: PerlinNoiseGenerator::new(seed + 8)
                .set_scale(16.)
                .set_lacunarity(2.0)
                .set_persistance(0.5)
                .set_octaves(8),
        }
    }

    /// Generates a chunk from its coordinates.
    /// Chunk dimension are
    /// (pos.0, pos.1)                 ...   (pos.0 + `chunk_size`, pos.1)
    ///                                ...
    /// (pos.0, pos.1 + `chunk_size`)  ...   (pos.0 + `chunk_size`, pos.1 `chunk_size`ze)
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn generate_chunk(&self, pos: (isize, isize)) -> Chunk {
        let pos = (pos.0 as f64, pos.1 as f64);

        let temp = self.temperature_noise.noise(pos);
        let moisture = self.moisture_noise.noise(pos);
        let continentalness = self.continentalness_noise.noise(pos);
        let erosion = self.erosion_noise.noise(pos);

        Chunk {
            temp,
            moisture,
            continentalness,
            erosion,
        }
    }
}
