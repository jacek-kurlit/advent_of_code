use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Add<Output = T> + PartialOrd,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn bounded_within(&self, min: T, max: T) -> bool {
        self.x >= min && self.x <= max && self.y >= min && self.y <= max
    }
}

impl<T> From<Vec3<T>> for Vec2<T>
where
    T: Add<Output = T> + PartialOrd,
{
    fn from(value: Vec3<T>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Add<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
