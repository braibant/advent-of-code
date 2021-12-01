#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

use std::ops::Add;

impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

use core::borrow::Borrow;

pub fn bounding_box<T, I>(mut it: I) -> Option<(Vector3<T>, Vector3<T>)>
where
    I: Iterator,
    I::Item: Borrow<Vector3<T>>,
    T: Ord + Copy,
{
    match it.next() {
        None => None,
        Some(p) => {
            let mut minx = p.borrow().x;
            let mut maxx = p.borrow().x;
            let mut miny = p.borrow().y;
            let mut maxy = p.borrow().y;
            let mut minz = p.borrow().z;
            let mut maxz = p.borrow().z;

            for p in it {
                minx = std::cmp::min(minx, p.borrow().x);
                miny = std::cmp::min(miny, p.borrow().y);
                minz = std::cmp::min(minz, p.borrow().z);

                maxx = std::cmp::max(maxx, p.borrow().x);
                maxy = std::cmp::max(maxy, p.borrow().y);
                maxz = std::cmp::max(minz, p.borrow().z);
            }
            Some((
                Vector3::new(minx, miny, minz),
                Vector3::new(maxx, maxy, maxz),
            ))
        }
    }
}
