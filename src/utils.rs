use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

pub use std::rc::Rc;

pub use num_traits::Float;

pub use crate::{color::*, interval::*, ray::*, vec3::*};

pub const PI: f64 = std::f64::consts::PI;

#[inline]
pub fn deg_to_rad<T>(deg: T) -> T
where
    T: Float,
{
    deg * T::from(PI).unwrap() / T::from(180.0).unwrap()
}

#[inline]
pub fn rand_float() -> f64 {
    let mut handle = rand::thread_rng();
    handle.gen()
}

#[inline]
pub fn rand_from_range<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    let mut handle = rand::thread_rng();
    handle.gen_range(range)
}
