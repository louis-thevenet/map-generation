use rand::{seq::SliceRandom, thread_rng};

#[derive(Default)]
pub struct PerlinNoiseGenerator {
    dimension: usize,
    permutations: Vec<usize>,
    octaves: usize,
}
impl PerlinNoiseGenerator {
    pub fn new(dimension: usize) -> Self {
        let mut permutations: Vec<usize> = (0..=(dimension)).collect::<Vec<usize>>();
        permutations.shuffle(&mut thread_rng());
        Self {
            dimension,
            permutations,
            ..Default::default()
        }
    }
    pub fn add_octaves(self, octaves: usize) -> Self {
        Self { octaves, ..self }
    }
    fn constant_vector(h: usize) -> Vector2 {
        match h % 4 {
            0 => Vector2(1., 1.),
            1 => Vector2(-1., 1.),
            2 => Vector2(-1., -1.),
            _ => Vector2(1., -1.),
        }
    }
    fn lerp(t: f64, a1: f64, a2: f64) -> f64 {
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

        let size = self.dimension;
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
        let mut amplitude = 1.0;
        let mut freq = 0.005;
        for _ in 0..self.octaves {
            let n = amplitude * self.perlin((pos.0 * freq, pos.1 * freq));
            result += n;
            amplitude *= 0.5;
            freq *= 2.0;
        }
        result
    }
    pub fn noise(&self, pos: (f64, f64)) -> f64 {
        if self.octaves == 0 {
            self.perlin(pos)
        } else {
            self.fractal_brownian_motion(pos)
        }
    }
}
struct Vector2(f64, f64);
impl Vector2 {
    fn dot_product(self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}
