pub use std::rc::Rc;

pub use num_traits::Float;

pub use crate::{color::*, interval::*, ray::*, vec3::*};

pub const PI: f64 = 3.1415926535897;

#[inline]
pub fn deg_to_rad<T>(deg: T) -> T
where
    T: Float,
{
    deg * T::from(PI).unwrap() / T::from(180.0).unwrap()
}
