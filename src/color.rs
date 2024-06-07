use crate::vec3::Vec3;
use std::ops::Mul;

pub type Color<T> = Vec3<T>;

pub fn write_color<T>(color: Color<T>)
where
    f64: Mul<T, Output = f64>,
{
    let (r, g, b) = (color.x, color.y, color.z);

    println!(
        "{} {} {}",
        (255.99 * r) as u32,
        (255.99 * g) as u32,
        (255.99 * b) as u32,
    );
}
