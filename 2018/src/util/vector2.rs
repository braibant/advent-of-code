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

use core::borrow::Borrow;

pub fn bounding_box<T, I>(mut it: I) -> Option<(Vector2<T>, Vector2<T>)>
where
    I: Iterator,
    I::Item: Borrow<Vector2<T>>,
    T: Ord + Copy,
{
    match it.next() {
        None => None,
        Some(p) => {
            let mut minx = p.borrow().x;
            let mut maxx = p.borrow().x;
            let mut miny = p.borrow().y;
            let mut maxy = p.borrow().y;
            for p in it {
                minx = std::cmp::min(minx, p.borrow().x);
                miny = std::cmp::min(miny, p.borrow().y);
                maxx = std::cmp::max(maxx, p.borrow().x);
                maxy = std::cmp::max(maxy, p.borrow().y);
            }
            Some((Vector2::new(minx, miny), Vector2::new(maxx, maxy)))
        }
    }
}
