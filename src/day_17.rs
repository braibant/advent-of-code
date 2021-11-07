use crate::util::vector2::Vector2;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

type Position = Vector2<i32>;

#[derive(Debug, Copy, Clone)]
enum Type {
    Clay,
    Water,
}

#[derive(Debug, Clone)]
struct T {
    scan: HashMap<Position, Type>,
    upper_left: Position,
    bottom_right: Position,
}

fn parse_line(s: &str) -> Vec<Position> {
    if s.starts_with('x') {
        let (x, y1, y2) = scan_fmt!(s, "x={}, y={}..{}", i32, i32, i32).unwrap();
        let (y1, y2) = (std::cmp::min(y1, y2), std::cmp::max(y1, y2));
        (y1..=y2).map(|y| Vector2::new(x, y)).collect()
    } else {
        let (y, x1, x2) = scan_fmt!(s, "y={}, x={}..{}", i32, i32, i32).unwrap();
        let (x1, x2) = (std::cmp::min(x1, x2), std::cmp::max(x1, x2));
        (x1..=x2).map(|x| Vector2::new(x, y)).collect()
    }
}
fn parse(s: &str) -> T {
    let mut t = HashMap::new();
    for line in s.lines() {
        for pos in parse_line(line) {
            t.insert(pos, Type::Clay);
        }
    }
    let (upper_left, bottom_right) = crate::util::vector2::bounding_box(t.keys()).unwrap();

    T {
        scan: t,
        upper_left,
        bottom_right,
    }
}

// --> x
// |
// v y
fn print(t: &T) {
    let mut b = String::new();
    b.push_str("   ");
    for x in t.upper_left.x..=t.bottom_right.x {
        b.push_str(&format!("{}", (x / 100) % 10));
    }
    b.push('\n');
    b.push_str("   ");
    for x in t.upper_left.x..=t.bottom_right.x {
        b.push_str(&format!("{}", (x / 10) % 10));
    }
    b.push('\n');
    b.push_str("   ");
    for x in t.upper_left.x..=t.bottom_right.x {
        b.push_str(&format!("{}", (x % 10)));
    }
    b.push('\n');

    for y in t.upper_left.y..=t.bottom_right.y {
        b.push_str(&format!("{:3}", y));
        for x in t.upper_left.x..=t.bottom_right.x {
            match t.scan.get(&Vector2::new(x, y)) {
                None => b.push('.'),
                Some(Type::Clay) => b.push('#'),
                Some(Type::Water) => b.push('~'),
            }
        }
        b.push('\n');
    }
    println!("{}", b);
}

const D: Vector2<i32> = Vector2 { x: 0, y: 1 };
const L: Vector2<i32> = Vector2 { x: -1, y: 0 };
const R: Vector2<i32> = Vector2 { x: 1, y: 0 };

// The goal of part 1 is to figure out how many tile the water can reach,
// knowing that there is a spring at x=500, y=0. Water flows down, until it
// reaches a level of clay. What happens next depends on the topology of the
// clay. Let's look left and right (while there is clay under the level we
// explore), until we reach a gap, or until we reach clay boundaries. If we
// reach clay boundaries, mark this level as filled with water and iterate. If
// we reach a gap on a single side, mark the tile "after the ledge" as a spring.
// If we reach a gap on both sides, mark the tiles "after the ledges" as
// springs. Since we have an infinite supply of water, we can just iterate from
// the springs that were created at the previous step. Each spring will create
// at most two other springs.
fn down(t: &T, mut pos: Position) -> Option<Position> {
    loop {
        if pos.y > t.bottom_right.y {
            return None;
        } else if t.scan.get(&(pos + D)).is_some() {
            return Some(pos);
        } else {
            pos = pos + D
        }
    }
}

// The result of walking left / right is either that we find a wall at a given
// position, or that there is a hole under a given position.
enum Border {
    Wall(Position),
    Hole(Position),
}

fn left(t: &T, mut pos: Position) -> Border {
    loop {
        if t.scan.get(&(pos + D)).is_none() {
            return Border::Hole(pos);
        } else if t.scan.get(&(pos + L)).is_some() {
            return Border::Wall(pos + L);
        } else {
            pos = pos + L;
        }
    }
}

fn right(t: &T, mut pos: Position) -> Border {
    loop {
        if t.scan.get(&(pos + D)).is_none() {
            return Border::Hole(pos);
        } else if t.scan.get(&(pos + R)).is_some() {
            return Border::Wall(pos + R);
        } else {
            pos = pos + R;
        }
    }
}

use std::collections::HashSet;
fn pour(t: &mut T, p: Position) {
    use Border::{Hole, Wall};

    let mut progress = true;

    while progress {
        let mut todo = vec![p];
        let mut visited = HashSet::new();
        progress = false;
        while let Some(p) = todo.pop() {
            // println!("{:?}", p);
            // print(&t);
            if t.scan.get(&p).is_none() && p.y <= t.bottom_right.y && !visited.contains(&p) {
                visited.insert(p);
                match down(t, p) {
                    None => {}
                    Some(p1) => match (left(t, p1), right(t, p1)) {
                        (Wall(l), Wall(r)) => {
                            // println!("{:?} -> W{:?}W", p, p1);
                            assert!(l.y == r.y);
                            progress = true;
                            for x in (l.x + 1)..r.x {
                                t.scan.insert(Vector2::new(x, l.y), Type::Water);
                            }
                            // todo.push(p)
                        }
                        (Hole(l), Wall(_)) => todo.push(l),
                        (Wall(_), Hole(r)) => {
                            todo.push(r);
                        }
                        (Hole(l), Hole(r)) => {
                            todo.push(l);
                            todo.push(r)
                        }
                    },
                }
            }
        }
    }
}

fn part1(t: &mut T) -> usize {
    pour(t, Vector2::new(500, 0));
    t.scan.values().filter(|t| matches!(t, Type::Water)).count()
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut t = parse(&contents);
    println!("{:?} {:?}", t.upper_left, t.bottom_right);
    println!("{}", part1(&mut t));
    print(&t);
}

#[cfg(test)]
mod tests {
    const S: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    use super::*;

    #[test]
    fn test_example() {
        let mut t = parse(S);
        let r = part1(&mut t);
        print(&t);
        assert_eq!(r, 57)
    }
}
