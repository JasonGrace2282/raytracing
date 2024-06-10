use crate::{
    hit::HitRecord,
    utils::{rand_in_unit_sphere, reflect, Color, Float, Ray, Vec3},
};
use std::fmt;

pub trait Material<T> {
    fn scatter(&self, ray_in: &Ray<T>, record: &HitRecord<T>) -> Option<(Color<T>, Ray<T>)>;
}

impl<T> fmt::Debug for dyn Material<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material")
    }
}

#[derive(Debug)]
pub struct Lambertian<T> {
    albedo: Color<T>,
}

impl<T> Lambertian<T> {
    pub fn new(albedo: Color<T>) -> Lambertian<T> {
        Self { albedo }
    }
}

impl<T> Material<T> for Lambertian<T>
where
    T: Float,
    Vec3<T>: From<Vec3<f64>>,
{
    fn scatter(&self, ray_in: &Ray<T>, record: &HitRecord<T>) -> Option<(Color<T>, Ray<T>)> {
        let mut scatter_direction =
            record.normal + Into::<Vec3<T>>::into(rand_in_unit_sphere()).unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Debug)]
pub struct Metal<T> {
    albedo: Vec3<T>,
}

impl<T> Metal<T> {
    pub fn new(albedo: Color<T>) -> Metal<T> {
        Self { albedo }
    }
}

impl<T> Material<T> for Metal<T>
where
    T: Float,
{
    fn scatter(&self, ray_in: &Ray<T>, record: &HitRecord<T>) -> Option<(Color<T>, Ray<T>)> {
        let reflected = reflect(*ray_in.get_direction(), record.normal);
        let scattered = Ray::new(record.point, reflected);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
