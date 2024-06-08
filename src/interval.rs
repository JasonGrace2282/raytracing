use num_traits::Float;

#[derive(Debug, Copy, Clone)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T> {
    pub fn new(min: T, max: T) -> Interval<T> {
        Self { min, max }
    }
}

impl<T> Interval<T>
where
    T: Float,
{
    pub fn empty() -> Interval<T> {
        Self::new(T::infinity(), -T::infinity())
    }

    pub fn universe() -> Interval<T> {
        Self::new(-T::infinity(), T::infinity())
    }

    pub fn size(&self) -> T {
        self.max - self.min
    }

    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }
}
