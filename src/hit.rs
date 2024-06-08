use crate::utils::{
    Ray,
    Point,
    Vec3,
    Rc,
};
use num_traits::Float;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord<T> {
    pub point: Point<T>,
    pub normal: Vec3<T>,
    pub t: T,
    front: Option<bool>,
}

impl<T> HitRecord<T>
where
    T: Copy + Float
{
    pub fn new(point: Point<T>, normal: Vec3<T>, t: T, ray: &Ray<T>) -> HitRecord<T>
    {
        let mut instance = Self {
            point,
            normal,
            t,
            front: None,
        };
        instance.set_front_face(ray, normal);
        instance
    }

    pub fn front_face(&self) -> bool {
        self.front.expect("ERROR:: Failed to call set_front_face before getting value")
    }

    pub fn set_front_face(&mut self, ray: &Ray<T>, outward_normal: Vec3<T>)
    {
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
    fn hit(&self, ray: &Ray<T>, ray_tmin: T, ray_tmax: T) -> Option<HitRecord<T>>;
}

pub struct HittableList<T> {
    objects: Vec<Rc<dyn Hittable<T>>>
}

impl<T> HittableList<T> {
    pub fn new() -> HittableList<T> {
        Self {
            objects: vec![]
        }
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
    T: Copy
{
    fn hit(&self, ray: &Ray<T>, ray_tmin: T, ray_tmax: T) -> Option<HitRecord<T>>
    {
        let mut record: Option<HitRecord<T>> = None;
        let mut closest = ray_tmax;

        for object in self.objects.iter() {
            if let Some(r) = object.hit(ray, ray_tmin, closest) {
                record = Some(r);
                closest = record.unwrap().t;
            }
        }

        record
    }
}
