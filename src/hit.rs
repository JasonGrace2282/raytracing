use crate::{material::Material, utils::{Interval, Point, Ray, Rc, Vec3}};
use num_traits::Float;

#[derive(Debug, Clone)]
pub struct HitRecord<T> {
    pub point: Point<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub material: Rc<dyn Material<T>>,
    front: Option<bool>,
}

impl<T> HitRecord<T>
where
    T: Copy + Float,
{
    pub fn new(point: Point<T>, normal: Vec3<T>, t: T, ray: &Ray<T>, material: Rc<dyn Material<T>>) -> HitRecord<T> {
        let mut instance = Self {
            point,
            normal,
            t,
            front: None,
            material,
        };
        instance.set_front_face(ray, normal);
        instance
    }

    pub fn front_face(&self) -> bool {
        self.front
            .expect("ERROR:: Failed to call set_front_face before getting value")
    }

    pub fn set_front_face(&mut self, ray: &Ray<T>, outward_normal: Vec3<T>) {
        self.front = Some(ray.get_direction().dot(&outward_normal) < T::from(0).unwrap());
        if let Some(f) = self.front {
            self.normal = match f {
                true => outward_normal,
                false => outward_normal * T::from(-1).unwrap(),
            }
        }
    }
}

pub trait Hittable<T> {
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>>;
}

pub struct HittableList<T> {
    objects: Vec<Rc<dyn Hittable<T>>>,
}

impl<T> Default for HittableList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HittableList<T> {
    pub fn new() -> HittableList<T> {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable<T>>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T> Hittable<T> for HittableList<T>
where
    T: Copy,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>> {
        let mut record: Option<HitRecord<T>> = None;
        let mut closest: T = ray_t.max;

        for object in self.objects.iter() {
            let interval = Interval::new(ray_t.min, closest);
            if let Some(r) = object.hit(ray, interval) {
                record = Some(r.clone());
                closest = record.as_ref().unwrap().t;
            }
        }

        record
    }
}
