use super::Vector2;

/// Offset used to handle negative positions
const POS_OFFSET: f64 = 1024.0;

#[derive(Default, Debug)]
pub struct PerlinNoiseGenerator {
    lacunarity: f64,
    octaves: usize,
    persistance: f64,
}
impl PerlinNoiseGenerator {
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
    pub fn lerp(t: f64, a1: f64, a2: f64) -> f64 {
        t.mul_add(a2 - a1, a1)
    }

    fn fade(t: f64) -> f64 {
        6.0f64.mul_add(t, -15.).mul_add(t, 10.) * t * t * t
    }
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::similar_names
    )]
    fn perlin(pos: (f64, f64), permutations: &[usize]) -> f64 {
        let (x, y) = (pos.0 + POS_OFFSET, pos.1 + POS_OFFSET);
        let (nx, ny) = ((x.floor()) as usize, (y.floor()) as usize);
        let (fx, fy) = (x - x.floor(), y - y.floor());

        let tr = Vector2(fx - 1.0, fy - 1.0);
        let tl = Vector2(fx, fy - 1.0);
        let br = Vector2(fx - 1.0, fy);
        let bl = Vector2(fx, fy);

        let size = permutations.len();
        let v_tr = permutations[(permutations[(nx + 1) % size] + (ny + 1) % size) % size];
        let v_tl = permutations[(permutations[nx % size] + (ny + 1) % size) % size];
        let v_br = permutations[(permutations[(nx + 1) % size] + ny % size) % size];
        let v_bl = permutations[(permutations[nx % size] + ny % size) % size];

        let d_tr = tr.dot_product(&Self::constant_vector(v_tr));
        let d_tl = tl.dot_product(&Self::constant_vector(v_tl));
        let d_br = br.dot_product(&Self::constant_vector(v_br));
        let d_bl = bl.dot_product(&Self::constant_vector(v_bl));

        let u = Self::fade(fx);
        let v = Self::fade(fy);

        Self::lerp(u, Self::lerp(v, d_bl, d_tl), Self::lerp(v, d_br, d_tr))
    }
    fn fractal_brownian_motion(&self, pos: (f64, f64), scale: f64, permutations: &[usize]) -> f64 {
        let mut result = 0.0;
        for oct in 0..self.octaves {
            let freq = self.lacunarity.powi(oct.try_into().unwrap());
            let amplitude = self.persistance.powi(oct.try_into().unwrap());
            result += amplitude
                * PerlinNoiseGenerator::perlin(
                    (pos.0 * freq / scale, pos.1 * freq / scale),
                    permutations,
                );
        }
        result
    }
    #[must_use]
    /// Generate noise from coordinates.
    pub fn noise(&self, pos: (f64, f64), scale: f64, permutations: &[usize]) -> f64 {
        if self.octaves == 0 {
            PerlinNoiseGenerator::perlin(pos, permutations)
        } else {
            self.fractal_brownian_motion(pos, scale, permutations)
        }
    }
}
