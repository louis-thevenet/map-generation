use rand::{seq::SliceRandom, thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::chunk::Chunk;

use super::noise_to_map::NoiseToMap;

/// Length of the permutation vector. We should't see repetition before >~20k pixels.
const PERMUTATION_LENGTH: usize = 1024 * 16;

/// So that 1.0 is a good scale.
const DEFAULT_SCALE: f64 = 40.0;

#[derive(Default, Debug)]
/// Represents a perlin noise generator with its settings.
pub struct TerrainGenerator {
    noise_to_map: NoiseToMap,
    chunk_size: usize,
    lacunarity: f64,
    octaves: usize,
    permutations: Vec<usize>,
    persistance: f64,
    scale: f64,
}
impl TerrainGenerator {
    #[must_use]
    /// Creates a new `PerlinNoiseGenerator` from a `chunk_size` and an optional `seed`
    pub fn new(chunk_size: usize, seed: Option<u64>) -> Self {
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
            permutations,
            ..Default::default()
        }
    }
    #[must_use]
    pub fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }
    #[must_use]
    pub fn set_persistance(self, persistance: f64) -> Self {
        Self {
            persistance,
            ..self
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
    pub fn set_octaves(self, octaves: usize) -> Self {
        Self { octaves, ..self }
    }
    #[must_use]
    pub fn set_noise_to_map(self, noise_to_map: NoiseToMap) -> Self {
        Self {
            noise_to_map,
            ..self
        }
    }
    fn constant_vector(h: usize) -> Vector2 {
        match h % 4 {
            0 => Vector2(1., 1.),
            1 => Vector2(-1., 1.),
            2 => Vector2(-1., -1.),
            _ => Vector2(1., -1.),
        }
    }
    #[must_use]
    pub fn lerp(t: f64, a1: f64, a2: f64) -> f64 {
        a1 + t * (a2 - a1)
    }

    fn fade(t: f64) -> f64 {
        (((6. * t - 15.) * t) + 10.) * t * t * t
    }
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::similar_names
    )]
    fn perlin(&self, pos: (f64, f64)) -> f64 {
        let (x, y) = pos;
        let (nx, ny) = (pos.0 as usize, pos.1 as usize);
        let (fx, fy) = (x - x.floor(), y - y.floor());

        let tr = Vector2(fx - 1.0, fy - 1.0);
        let tl = Vector2(fx, fy - 1.0);
        let br = Vector2(fx - 1.0, fy);
        let bl = Vector2(fx, fy);

        let size = self.permutations.len();
        let v_tr = self.permutations[(self.permutations[(nx + 1) % size] + (ny + 1) % size) % size];
        let v_tl = self.permutations[(self.permutations[nx % size] + (ny + 1) % size) % size];
        let v_br = self.permutations[(self.permutations[(nx + 1) % size] + ny % size) % size];
        let v_bl = self.permutations[(self.permutations[nx % size] + ny % size) % size];

        let d_tr = tr.dot_product(&Self::constant_vector(v_tr));
        let d_tl = tl.dot_product(&Self::constant_vector(v_tl));
        let d_br = br.dot_product(&Self::constant_vector(v_br));
        let d_bl = bl.dot_product(&Self::constant_vector(v_bl));

        let u = Self::fade(fx);
        let v = Self::fade(fy);

        Self::lerp(u, Self::lerp(v, d_bl, d_tl), Self::lerp(v, d_br, d_tr))
    }
    fn fractal_brownian_motion(&self, pos: (f64, f64)) -> f64 {
        let mut result = 0.0;
        for oct in 0..self.octaves {
            let freq = self.lacunarity.powi(oct.try_into().unwrap());
            let amplitude = self.persistance.powi(oct.try_into().unwrap());
            result +=
                amplitude * self.perlin((pos.0 * freq / self.scale, pos.1 * freq / self.scale));
        }
        result
    }
    #[must_use]
    /// Generate noise from coordinates.
    pub fn noise(&self, pos: (f64, f64)) -> f64 {
        if self.octaves == 0 {
            self.perlin(pos)
        } else {
            self.fractal_brownian_motion(pos)
        }
    }

    /// Generates a chunk from its coordinates.
    /// Chunk dimension are
    /// (pos.0, pos.1)                 ...   (pos.0 + `chunk_size`, pos.1)
    ///                                ...
    /// (pos.0, pos.1 + `chunk_size`)  ...   (pos.0 + `chunk_size`, pos.1 `chunk_size`ze)
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn generate_chunk(&self, pos: (usize, usize)) -> Chunk {
        let x = (pos.0 * self.chunk_size) as f64;
        let y = (pos.1 * self.chunk_size) as f64;
        let mut result = vec![vec![0.0; self.chunk_size]; self.chunk_size];

        result.par_iter_mut().enumerate().for_each(|(y_offset, v)| {
            v.par_iter_mut().enumerate().for_each(move |(x_offset, v)| {
                *v = self.noise((x + x_offset as f64, y + y_offset as f64));
            });
        });
        self.noise_to_map.chunk_from_noise(result)
    }
}
struct Vector2(f64, f64);
impl Vector2 {
    fn dot_product(self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}
