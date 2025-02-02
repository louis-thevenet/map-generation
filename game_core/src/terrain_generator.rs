pub mod noise_to_map;
pub mod perlin_noise;

/// So that 1.0 is a good scale.
const DEFAULT_SCALE: f64 = 40.0;
/// Length of the permutation vector. We should't see repetition before >~20k pixels.
const PERMUTATION_LENGTH: usize = 1024 * 16;

/// Lower bound for temperature
pub const TEMPERATURE_LOWER_BOUND: f64 = -10.0;
/// Upper bound for temperature
pub const TEMPERATURE_UPPER_BOUND: f64 = 40.0;
use noise_to_map::NoiseToMap;
use perlin_noise::PerlinNoiseGenerator;
use rand::{seq::SliceRandom, thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::chunk::Chunk;

#[derive(Default, Debug)]
/// Represents a perlin noise generator with its settings.
pub struct TerrainGenerator {
    scale: f64,
    pub chunk_size: usize,
    pub terrain_noise_generator: PerlinNoiseGenerator,
    pub temperature_noise_generator: PerlinNoiseGenerator,
    permutations: Vec<usize>,
    noise_to_map: NoiseToMap,
}
impl TerrainGenerator {
    #[must_use]
    /// Creates a new `TerrainGenetor` from a `chunk_size` and an optional `seed`
    pub fn new(
        chunk_size: usize,
        seed: Option<u64>,
        terrain_noise_generator: PerlinNoiseGenerator,
        temperature_noise_generator: PerlinNoiseGenerator,
    ) -> Self {
        let mut permutations: Vec<usize> = (0..=PERMUTATION_LENGTH).collect::<Vec<usize>>();

        let seed_value = if let Some(seed_value) = seed {
            seed_value
        } else {
            let seed_value = thread_rng().next_u64();
            println!("Seed: {seed_value}");
            seed_value
        };
        let mut rng = ChaCha8Rng::seed_from_u64(seed_value);
        permutations.shuffle(&mut rng);
        Self {
            chunk_size,
            terrain_noise_generator,
            temperature_noise_generator,
            permutations,
            ..Default::default()
        }
    }
    #[must_use]
    pub fn set_scale(self, scale: f64) -> Self {
        Self {
            scale: scale * DEFAULT_SCALE,
            ..self
        }
    }
    #[must_use]
    pub fn set_terrain_noise_generator(
        self,
        terrain_noise_generator: PerlinNoiseGenerator,
    ) -> Self {
        Self {
            terrain_noise_generator,
            ..self
        }
    }
    #[must_use]
    pub fn set_noise_to_map(self, noise_to_map: NoiseToMap) -> Self {
        Self {
            noise_to_map,
            ..self
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
        let x = (pos.0 * self.chunk_size as isize) as f64;
        let y = (pos.1 * self.chunk_size as isize) as f64;
        let mut terrain_noise = vec![vec![0.0; self.chunk_size]; self.chunk_size];
        let mut temperature_noise = vec![vec![0.0; self.chunk_size]; self.chunk_size];

        terrain_noise
            .par_iter_mut()
            .enumerate()
            .for_each(|(y_offset, v)| {
                v.par_iter_mut().enumerate().for_each(move |(x_offset, v)| {
                    *v = self.terrain_noise_generator.noise(
                        (x + x_offset as f64, y + y_offset as f64),
                        self.scale,
                        &self.permutations,
                    );
                });
            });
        temperature_noise
            .par_iter_mut()
            .enumerate()
            .for_each(|(y_offset, v)| {
                v.par_iter_mut().enumerate().for_each(move |(x_offset, v)| {
                    *v = self.temperature_noise_generator.noise(
                        (x + x_offset as f64, y + y_offset as f64),
                        self.scale,
                        &self.permutations,
                    );
                });
            });
        self.noise_to_map
            .chunk_from_noise(&terrain_noise, &temperature_noise)
    }
}
struct Vector2(f64, f64);
impl Vector2 {
    fn dot_product(self, rhs: &Self) -> f64 {
        self.0.mul_add(rhs.0, self.1 * rhs.1)
    }
}
