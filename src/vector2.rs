#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Vector2 {
    pub x: i64,
    pub y: i64,
}

use std::ops::Add;
impl Add for Vector2 {
    type Output = Vector2;

    fn add(self: Vector2, b: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: i64, y: i64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn norm1(&self, other: &Vector2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
