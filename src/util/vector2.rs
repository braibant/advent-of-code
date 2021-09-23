#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

use std::ops::Add;

impl<T> Add for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn bounding_box<T>(points: &[Vector2<T>]) -> (Vector2<T>, Vector2<T>)
where
    T: Ord + Copy,
{
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();

    (Vector2::new(minx, miny), Vector2::new(maxx, maxy))
}
