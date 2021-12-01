#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

use std::ops::Add;
impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self: Vector2<T>, b: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl Vector2<i64> {
    pub fn norm1(&self, other: &Vector2<i64>) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
