use savefile::savefile_derive::Savefile;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Savefile)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T>> Add for Vector2D<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
