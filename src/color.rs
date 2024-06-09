use crate::{interval::Interval, utils::Float, vec3::Vec3};

pub type Color<T> = Vec3<T>;

fn gamma_correct<T: Float>(linear: T) -> T {
    if (linear > T::from(0.0).unwrap()) {
        return linear.sqrt();
    }
    T::from(0.0).unwrap()
}

pub fn write_color<T>(color: Color<T>)
where
    T: Float,
{
    let (r, g, b) = (
        gamma_correct(color.x),
        gamma_correct(color.y),
        gamma_correct(color.z),
    );
    let intensity: Interval<T> = Interval::new(T::from(0.0).unwrap(), T::from(0.99999).unwrap());
    let num = T::from(260.0).unwrap();

    let r: u32 = num_traits::cast(num * intensity.clamp(r)).unwrap();
    let g: u32 = num_traits::cast(num * intensity.clamp(g)).unwrap();
    let b: u32 = num_traits::cast(num * intensity.clamp(b)).unwrap();

    println!("{r} {g} {b}");
}
