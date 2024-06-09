use crate::{
    hit::{HitRecord, Hittable},
    utils::{Float, Interval, Point, Ray, Vec3},
};

#[derive(Debug, Copy, Clone)]
pub struct Sphere<T> {
    center: Point<T>,
    radius: T,
}

impl<T: Float> Sphere<T> {
    pub fn new(center: Vec3<T>, r: T) -> Sphere<T> {
        let mut radius = T::from(0).unwrap();
        if r > T::from(0).unwrap() {
            radius = r;
        }
        Self { center, radius }
    }
}

impl<T> Hittable<T> for Sphere<T>
where
    T: Copy + Float,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>> {
        let oc = self.center - *ray.get_origin();
        let a = ray.get_direction().length_squared();
        let h = ray.get_direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < T::from(0)? {
            return None;
        }
        let sqrt = discriminant.sqrt();
        let mut root = (h - sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        Some(HitRecord::new(
            point,
            (point - self.center) / self.radius,
            root,
            ray,
        ))
    }
}
