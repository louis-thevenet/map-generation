use image::{ImageBuffer, Rgb};
use rand::{seq::SliceRandom, thread_rng};
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

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn perlin(permutations: &[usize], pos: (f64, f64)) -> f64 {
    let (x, y) = pos;
    let (fx, fy) = (x.floor() as usize, y.floor() as usize);

    let TR = Vector((fx + 1) as f64, (fy + 1) as f64);
    let TL = Vector(fx as f64, (fy + 1) as f64);
    let BR = Vector((fx + 1) as f64, (fy) as f64);
    let BL = Vector(fx as f64, (fy) as f64);

    let vTR = permutations[permutations[fx + 1] + fy + 1];
    let vTL = permutations[permutations[fx] + fy + 1];
    let vBR = permutations[permutations[fx + 1] + fy];
    let vBL = permutations[permutations[fx] + fy];

    let dTR = TR.dot_product(&constant_vector(vTR));
    let dTL = TL.dot_product(&constant_vector(vTL));
    let dBR = BR.dot_product(&constant_vector(vBR));
    let dBL = BL.dot_product(&constant_vector(vBL));

    let u = fade(fx as f64);
    let v = fade(fy as f64);
    lerp(u, lerp(v, dBL, dTL), lerp(v, dBR, dTR))
}
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn main() {
    let mut permutations: Vec<usize> = (0..=256).collect::<Vec<usize>>();
    permutations.shuffle(&mut thread_rng());
    let mut img = ImageBuffer::new(256, 256);
    for y in 0..=255 {
        for x in 0..=255 {
            let pos = (f64::from(x) / 10., f64::from(y) / 10.);
            let noise = (perlin(&permutations, pos) + 1.) / 2.;
            let px = (noise * 255.) as u8;
            img.put_pixel(x, y, Rgb([px, px, px]));
        }
    }
    let _ = img.save("output.png");
}
