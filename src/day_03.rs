use std::collections::HashSet;
use std::ops::Add;

#[derive(Hash, PartialEq, Debug, Copy, Clone, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Hash, PartialEq, Debug, Copy, Clone, Eq)]
struct Vector2d {
    x: i64,
    y: i64,
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn dir(dir: Dir) -> Vector2d {
    match dir {
        Dir::U => Vector2d { x: 0, y: 1 },
        Dir::D => Vector2d { x: 0, y: -1 },
        Dir::L => Vector2d { x: -1, y: 0 },
        Dir::R => Vector2d { x: 1, y: 0 },
    }
}

fn scale(n: i64, v: Vector2d) -> Vector2d {
    Vector2d {
        x: n * v.x,
        y: n * v.y,
    }
}

fn distance(v: &Vector2d) -> i64 {
    v.x.abs() + v.y.abs()
}

// Parse each instruction into a direction and a magnitude
fn parse(s: &str) -> (Dir, i64) {
    let n: i64 = s[1..].parse().unwrap();
    match &s[0..1] {
        "R" => (Dir::R, n),
        "U" => (Dir::U, n),
        "D" => (Dir::D, n),
        "L" => (Dir::L, n),
        _ => panic!("Parse error: {}", s),
    }
}

mod part1 {
    // Test if [b] belongs to the segment denoted by a starting point [a], a direction and a length
    fn mem(a: &Vector2d, dir: Dir, n: i64, b: &Vector2d) -> bool {
        let Vector2d { x: xa, y: ya } = *a;
        let Vector2d { x: xb, y: yb } = *b;

        match dir {
            Dir::U => xa == xb && ya <= yb && yb <= ya + n,
            Dir::D => xa == xb && ya - n <= yb && yb <= ya,
            Dir::L => ya == yb && xa - n <= yb && xb <= xa,
            Dir::R => ya == yb && xa <= xb && xb <= xa + n,
        }
    }

    fn add_intersections(
        acc: &mut HashSet<Vector2d>,
        start_a: &Vector2d,
        dir_a: Dir,
        length_a: i64,
        start_b: &Vector2d,
        dir_b: Dir,
        length_b: i64,
    ) {
        let mut pt = start_b.clone();
        for i in 0..length_b + 1 {
            if mem(start_a, dir_a, length_a, &pt) {
                acc.insert(pt.clone());
            };
            pt = pt + dir(dir_b);
        }
    }

    fn build_path(wire: Vec<(Dir, i64)>) -> Vec<(Vector2d, Dir, i64)> {
        let mut acc = Vec::new();
        let mut pt = Vector2d { x: 0, y: 0 };
        for &(d, length) in wire.iter() {
            acc.push((pt.clone(), d, length));
            pt = pt + scale(length, dir(d))
        }
        acc
    }

    type Path = Vec<(Vector2d, Dir, i64)>;

    fn intersections(p1: &Path, p2: &Path) -> HashSet<Vector2d> {
        let mut acc = HashSet::new();
        let mut pt = Vector2d { x: 0, y: 0 };

        for &(start_a, dir_a, length_a) in p1.iter() {
            for &(start_b, dir_b, length_b) in p2.iter() {
                add_intersections(
                    &mut acc, &start_a, dir_a, length_a, &start_b, dir_b, length_b,
                )
            }
        }
        acc
    }
}

use part1::*;
pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let wires: Vec<_> = contents.split("\n").collect();

    let wire1: Vec<_> = wires[0].split(",").map(parse).collect();
    let wire2: Vec<_> = wires[1].split(",").map(parse).collect();

    let path1 = build_path(wire1);
    let path2 = build_path(wire2);

    let mut s = intersections(&path1, &path2);
    s.remove(&Vector2d { x: 0, y: 0 });
    let result = s.iter().min_by_key(|v| distance(v));
    println!("{:?}", result)
}
