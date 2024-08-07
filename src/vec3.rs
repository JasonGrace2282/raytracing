use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::utils::{rand_float, rand_from_range};

pub fn rand_in_unit_sphere() -> Vec3<f64> {
    loop {
        let p = Vec3::<f64>::rand_from_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn rand_in_unit_disk() -> Vec3<f64> {
    loop {
        let p = Vec3::new(rand_from_range(-1.0..1.0), rand_from_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3<f64>) -> Vec3<f64> {
    let on_unit_sphere = rand_in_unit_sphere().unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    }
    on_unit_sphere * -1.0
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    pub fn map<U, F>(self, mut f: F) -> Vec3<U>
    where
        F: FnMut(T) -> U,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn rand() -> Vec3<f64> {
        Vec3::new(rand_float(), rand_float(), rand_float())
    }

    pub fn rand_from_range(min: f64, max: f64) -> Vec3<f64> {
        Vec3::new(
            rand_from_range(min..max),
            rand_from_range(min..max),
            rand_from_range(min..max),
        )
    }
}

impl<T> Vec3<T>
where
    T: Clone,
{
    pub fn splat(v: T) -> Vec3<T> {
        Self::new(v.clone(), v.clone(), v)
    }
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub const fn to_array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn unit_vector(&self) -> Vec3<T> {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = T::from(1e-8).unwrap();
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn reflect(&self, n: Vec3<T>) -> Vec3<T> {
        *self - n * T::from(2.0).unwrap() * self.dot(&n)
    }

    #[inline]
    pub fn refract(&self, n: Vec3<T>, eta_by_etap: T) -> Vec3<T> {
        let cos_theta = [
            (*self * T::from(-1.0).unwrap()).dot(&n),
            T::from(1.0).unwrap(),
        ]
        .iter()
        .fold(T::infinity(), |a, &b| a.min(b));
        let r_out_perp = (*self + n * cos_theta) * eta_by_etap;
        let r_out_parallel =
            n * -((T::from(1.0).unwrap() - r_out_perp.length_squared()).abs()).sqrt();
        r_out_perp + r_out_parallel
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Vec3<T>
where
    T: Mul<T, Output = T> + Copy,
{
    pub fn mul_vec3(&self, other: Vec3<T>) -> Vec3<T> {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> MulAssign<T> for Vec3<T>
where
    T: Copy + MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T> DivAssign<T> for Vec3<T>
where
    T: Copy + DivAssign<T>,
{
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

pub type Point<T> = Vec3<T>;
