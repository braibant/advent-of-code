use std::collections::HashMap;
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

// This allocates memory linearly with respect to the distance travelled, which would be an issue if the problem input was simply scaled to use higher numerical constants. This is not an issue here, and allows for a simple solution. An "optimisation" would be to avoid building the path for the second wire, and simply record the set of intersections as we build it (this would cut memory usage in two, roughly). An actual optimisation would be to build the list of segments (as was initially done for part 1, with memory usage linear in the number of wire segments), compute intersections, then compute the number of steps used to reach each such intersection.
fn build(wire: Vec<(Dir, i64)>) -> HashMap<Vector2d, i64> {
    let mut acc = HashMap::new();
    let mut pt = Vector2d { x: 0, y: 0 };
    let mut step = 0;

    for (d, length) in wire.iter() {
        let dir = dir(*d);
        for _ in 0..*length {
            if !acc.contains_key(&pt) {
                acc.insert(pt, step);
            };
            pt = pt + dir;
            step += 1;
        }
    }
    acc
}

fn intersections(p1: &HashMap<Vector2d, i64>, p2: &HashMap<Vector2d, i64>) -> HashSet<Vector2d> {
    let s1: HashSet<_> = p1.keys().cloned().collect();
    let s2: HashSet<_> = p2.keys().cloned().collect();
    s1.intersection(&s2).cloned().collect()
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let wires: Vec<_> = contents.split("\n").collect();

    let wire1: Vec<_> = wires[0].split(",").map(parse).collect();
    let wire2: Vec<_> = wires[1].split(",").map(parse).collect();

    let path1 = build(wire1);
    let path2 = build(wire2);
    let mut s = intersections(&path1, &path2);
    s.remove(&Vector2d { x: 0, y: 0 });
    let result = s.iter().min_by_key(|v| distance(v));
    println!("{:?}", result);

    let pt = s.iter().min_by_key(|pt| path1[pt] + path2[pt]).unwrap();
    println!("{:?}", path1[pt] + path2[pt]);
}
