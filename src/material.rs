use crate::{
    hit::HitRecord,
    utils::{rand_float, rand_in_unit_sphere, Color, Float, Ray, Vec3},
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
    T: Float + From<f64>,
    Vec3<T>: From<Vec3<f64>>,
{
    fn scatter(&self, _ray_in: &Ray<T>, record: &HitRecord<T>) -> Option<(Color<T>, Ray<T>)> {
        let mut scatter_direction =
            record.normal + rand_in_unit_sphere().map(Into::into).unit_vector();

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
    fuzziness: T,
}

impl<T> Metal<T> {
    pub fn new(albedo: Color<T>, fuzziness: Option<T>) -> Metal<T>
    where
        T: Float,
    {
        Self {
            albedo,
            fuzziness: fuzziness.unwrap_or(T::from(1.0).unwrap()),
        }
    }
}

impl<T> Material<T> for Metal<T>
where
    T: Float + From<f64>,
{
    fn scatter(&self, ray_in: &Ray<T>, record: &HitRecord<T>) -> Option<(Color<T>, Ray<T>)> {
        let mut reflected = ray_in.get_direction().reflect(record.normal);
        reflected =
            reflected.unit_vector() + rand_in_unit_sphere().map(Into::into) * self.fuzziness;
        let scattered = Ray::new(record.point, reflected);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Debug)]
pub struct Dielectric<T> {
    refraction_idx: T,
}

impl<T> Dielectric<T> {
    pub fn new(refraction_idx: T) -> Self {
        Self { refraction_idx }
    }
}

impl<T> Dielectric<T>
where
    T: Float,
{
    fn reflectance(cosine: T, reflection_index: T) -> T {
        let mut r0 =
            (T::from(1.0).unwrap() - reflection_index) / (T::from(1.0).unwrap() + reflection_index);
        r0 = r0 * r0;
        r0 + (T::from(1.0).unwrap() - r0) * (T::from(1.0).unwrap() - cosine).powi(5)
    }
}

impl<T> Material<T> for Dielectric<T>
where
    T: Float,
{
    fn scatter(&self, ray_in: &Ray<T>, record: &HitRecord<T>) -> Option<(Color<T>, Ray<T>)> {
        let attenuation: Vec3<T> = Color::new(
            T::from(1.0).unwrap(),
            T::from(1.0).unwrap(),
            T::from(1.0).unwrap(),
        );

        let unit_vector = ray_in.get_direction().unit_vector();

        let cos_theta = [
            (unit_vector * T::from(-1).unwrap()).dot(&record.normal),
            T::from(1.0).unwrap(),
        ]
        .iter()
        .fold(T::infinity(), |a, &b| a.min(b));
        let sin_theta = (T::from(1.0).unwrap() - cos_theta * cos_theta).sqrt();

        let ri = if record.front_face() {
            T::from(1.0).unwrap() / self.refraction_idx
        } else {
            self.refraction_idx
        };

        // no solution, so cannot refract in some cases
        let direction = if ri * sin_theta > T::from(1.0).unwrap()
            || Self::reflectance(cos_theta, ri) > T::from(rand_float()).unwrap()
        {
            unit_vector.reflect(record.normal)
        } else {
            unit_vector.refract(record.normal, ri)
        };

        let ray = Ray::new(record.point, direction);
        Some((attenuation, ray))
    }
}
