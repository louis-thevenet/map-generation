use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Offset used to handle negative positions
const POS_OFFSET: f64 = 1024.0;
/// Length of the permutation vector. We should't see repetition before >~20k pixels.
const PERMUTATION_LENGTH: usize = 1024 * 16;
pub struct Vector2(f64, f64);
impl Vector2 {
    fn dot_product(self, rhs: &Self) -> f64 {
        self.0.mul_add(rhs.0, self.1 * rhs.1)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PerlinNoiseGenerator {
    permutations: Vec<usize>,
    lacunarity: f64,
    octaves: usize,
    persistence: f64,
}
impl PerlinNoiseGenerator {
    pub fn new(seed: u64) -> Self {
        let mut permutations: Vec<usize> = (0..=PERMUTATION_LENGTH).collect::<Vec<usize>>();

        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        permutations.shuffle(&mut rng);
        Self {
            permutations,
            ..Default::default()
        }
    }
    #[must_use]
    pub fn set_lacunarity(self, lacunarity: f64) -> Self {
        Self { lacunarity, ..self }
    }
    #[must_use]
    pub fn set_persistence(self, persistence: f64) -> Self {
        Self {
            persistence,
            ..self
        }
    }
    #[must_use]
    pub fn set_octaves(self, octaves: usize) -> Self {
        Self { octaves, ..self }
    }
    const fn constant_vector(h: usize) -> Vector2 {
        match h % 4 {
            0 => Vector2(1., 1.),
            1 => Vector2(-1., 1.),
            2 => Vector2(-1., -1.),
            _ => Vector2(1., -1.),
        }
    }
    #[must_use]
    fn lerp(t: f64, a1: f64, a2: f64) -> f64 {
        t.mul_add(a2 - a1, a1)
    }

    fn fade(t: f64) -> f64 {
        6.0f64.mul_add(t, -15.).mul_add(t, 10.) * t * t * t
    }
    #[allow(clippy::similar_names)]
    fn perlin(&self, pos: (f64, f64)) -> f64 {
        let (x, y) = (pos.0 + POS_OFFSET, pos.1 + POS_OFFSET);
        #[allow(clippy::cast_possible_truncation)]
        let (nx, ny) = (
            ((x.floor()) as isize).unsigned_abs(),
            ((y.floor()) as isize).unsigned_abs(),
        );
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
    fn fractal_brownian_motion(&self, pos: (f64, f64), scale: f64) -> f64 {
        let mut result = 0.0;
        for oct in 0..self.octaves {
            let freq = self.lacunarity.powi(oct.try_into().unwrap());
            let amplitude = self.persistence.powi(oct.try_into().unwrap());
            result += amplitude * self.perlin((pos.0 * freq / scale, pos.1 * freq / scale));
        }
        result
    }
    #[must_use]
    /// Generate noise from coordinates.
    pub fn noise(&self, pos: (f64, f64), scale: f64) -> f64 {
        if self.octaves == 0 {
            self.perlin(pos)
        } else {
            self.fractal_brownian_motion(pos, scale)
        }
    }
}
