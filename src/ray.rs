use crate::vec3::{Point, Vec3};
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Ray<T> {
    origin: Point<T>,
    dir: Vec3<T>,
}

impl<T> Ray<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    pub fn new(origin: Point<T>, dir: Vec3<T>) -> Self {
        Self { origin, dir }
    }

    pub fn get_origin(&self) -> &Point<T> {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vec3<T> {
        &self.dir
    }

    pub fn at(&self, t: T) -> Vec3<T> {
        self.origin + self.dir * t
    }
}
