use image::{ImageBuffer, Rgb};
use rand::{seq::SliceRandom, thread_rng};
use rayon::iter::ParallelIterator;
struct Vector(f64, f64);
impl Vector {
    fn dot_product(self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}
fn constant_vector(h: usize) -> Vector {
    match h % 4 {
        0 => Vector(1., 1.),
        1 => Vector(-1., 1.),
        2 => Vector(-1., -1.),
        _ => Vector(1., -1.),
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
fn perlin(permutations: &[usize], pos: (f64, f64)) -> f64 {
    let (x, y) = pos;
    let (nx, ny) = (pos.0 as usize, pos.1 as usize);
    let (fx, fy) = (x - x.floor(), y - y.floor());

    let tr = Vector(fx - 1.0, fy - 1.0);
    let tl = Vector(fx, fy - 1.0);
    let br = Vector(fx - 1.0, fy);
    let bl = Vector(fx, fy);

    let size = permutations.len();
    let v_tr = permutations[(permutations[(nx + 1) % size] + (ny + 1) % size) % size];
    let v_tl = permutations[(permutations[nx % size] + (ny + 1) % size) % size];
    let v_br = permutations[(permutations[(nx + 1) % size] + ny % size) % size];
    let v_bl = permutations[(permutations[nx % size] + ny % size) % size];

    let d_tr = tr.dot_product(&constant_vector(v_tr));
    let d_tl = tl.dot_product(&constant_vector(v_tl));
    let d_br = br.dot_product(&constant_vector(v_br));
    let d_bl = bl.dot_product(&constant_vector(v_bl));

    let u = fade(fx);
    let v = fade(fy);

    lerp(u, lerp(v, d_bl, d_tl), lerp(v, d_br, d_tr))
}
fn fractal_brownian_motion(permutations: &[usize], pos: (f64, f64), num_octaves: usize) -> f64 {
    let mut result = 0.0;
    let mut amplitude = 1.0;
    let mut freq = 0.005;
    for _ in 0..num_octaves {
        let n = amplitude * perlin(permutations, (pos.0 * freq, pos.1 * freq));
        result += n;
        amplitude *= 0.5;
        freq *= 2.0;
    }
    result
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn main() {
    const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
    const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
    const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
    const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);
    let dimension = 1024 * 8;
    let mut permutations: Vec<usize> = (0..=(dimension as usize)).collect::<Vec<usize>>();
    permutations.shuffle(&mut thread_rng());
    let mut img = ImageBuffer::new(dimension, dimension);
    img.par_enumerate_pixels_mut().for_each(|(x, y, p)| {
        let pos = (f64::from(x), f64::from(y));

        let noise = (fractal_brownian_motion(&permutations, pos, 8) + 1.) / 2.;

        let px = if noise > 0.85 {
            WHITE
        } else if noise > 0.5 {
            GREEN
        } else if noise > 0.48 {
            YELLOW
        } else {
            BLUE
        };

        *p = px;
    });
    let _ = img.save("output.png");
}
