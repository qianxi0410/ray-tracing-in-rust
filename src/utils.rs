use rand::distributions::Uniform;
use rand::{self, Rng};
use std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // [0, 1)
    lazy_static::lazy_static! {
        static ref distribution: Uniform<f64> = Uniform::new(0.0, 1.0);
    }
    let mut rng = rand::thread_rng();
    rng.sample(*distribution)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    // [min, max)
    if max - min <= f64::EPSILON {
        min
    } else {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..max)
    }
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
