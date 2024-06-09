use crate::{interval::Interval, utils::Float, vec3::Vec3};

pub type Color<T> = Vec3<T>;

pub fn write_color<T>(color: Color<T>)
where
    T: Float,
{
    let (r, g, b) = (color.x, color.y, color.z);
    let intensity: Interval<T> = Interval::new(T::from(0.0).unwrap(), T::from(0.99999).unwrap());
    let num = T::from(255.99).unwrap();

    let r: u32 = num_traits::cast(num * intensity.clamp(r)).unwrap();
    let g: u32 = num_traits::cast(num * intensity.clamp(g)).unwrap();
    let b: u32 = num_traits::cast(num * intensity.clamp(b)).unwrap();

    println!("{r} {g} {b}");
}
