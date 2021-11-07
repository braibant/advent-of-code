use crate::util::vector2::Vector2;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

type Position = Vector2<i32>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Type {
    Clay,
    Water,
    Transient,
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
                Some(Type::Transient) => b.push('|'),
            }
        }
        b.push('\n');
    }
    println!("{}", b);
}

const D: Vector2<i32> = Vector2 { x: 0, y: 1 };
const L: Vector2<i32> = Vector2 { x: -1, y: 0 };
const R: Vector2<i32> = Vector2 { x: 1, y: 0 };

impl T {
    fn solid(&self, pos: Position) -> bool {
        matches!(self.scan.get(&(pos)), Some(Type::Water) | Some(Type::Clay))
    }

    fn transient(&self, pos: Position) -> bool {
        matches!(self.scan.get(&(pos)), Some(Type::Transient))
    }

    fn insert(&mut self, pos: Position, ty: Type) {
        self.scan.insert(pos, ty);
        if pos.x < self.upper_left.x {
            self.upper_left.x = pos.x
        };
        if self.bottom_right.x < pos.x {
            self.bottom_right.x = pos.x
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
        if !t.solid(pos + D) {
            return Border::Hole(pos);
        } else if t.solid(pos + L) {
            return Border::Wall(pos + L);
        } else {
            pos = pos + L;
        }
    }
}

fn right(t: &T, mut pos: Position) -> Border {
    loop {
        if !t.solid(pos + D) {
            return Border::Hole(pos);
        } else if t.solid(pos + R) {
            return Border::Wall(pos + R);
        } else {
            pos = pos + R;
        }
    }
}

fn pour(t: &mut T, spring: Position) {
    use Border::{Hole, Wall};

    t.insert(spring, Type::Transient);

    let mut y = spring.y;

    while y <= t.bottom_right.y {
        let mut candidates = vec![];

        for x in t.upper_left.x..=t.bottom_right.x {
            let p = Vector2::new(x, y);
            if t.transient(p) && !t.solid(p + D) {
                candidates.push(p + D)
            }
        }
        let mut backtrack = false;
        for p in candidates {
            match (left(t, p), right(t, p)) {
                (Wall(l), Wall(r)) => {
                    assert!(l.y == r.y);
                    for x in (l.x + 1)..r.x {
                        t.insert(Vector2::new(x, l.y), Type::Water);
                    }
                    backtrack = true;
                }
                (Hole(l), Wall(r)) => {
                    for x in (l.x)..r.x {
                        t.insert(Vector2::new(x, l.y), Type::Transient);
                    }
                }
                (Wall(l), Hole(r)) => {
                    for x in (l.x + 1)..=r.x {
                        t.insert(Vector2::new(x, l.y), Type::Transient);
                    }
                }
                (Hole(l), Hole(r)) => {
                    for x in l.x..=r.x {
                        t.insert(Vector2::new(x, l.y), Type::Transient);
                    }
                }
            }
        }
        if backtrack {
            y -= 2;
        } else {
            y += 1
        }
    }

    // while progress {
    //     let mut todo = vec![p];
    //     let mut visited = HashSet::new();
    //     progress = false;
    //     while let Some(p) = todo.pop() {
    //         // println!("{:?}", p);
    //         // print(&t);
    //         if t.scan.get(&p).is_none() && p.y <= t.bottom_right.y && !visited.contains(&p) {
    //             visited.insert(p);
    //             match down(t, p) {
    //                 None => {}
    //                 Some(p1) => match (left(t, p1), right(t, p1)) {
    //                     (Wall(l), Wall(r)) => {
    //                         // println!("{:?} -> W{:?}W", p, p1);
    //                         assert!(l.y == r.y);
    //                         progress = true;
    //                         for x in (l.x + 1)..r.x {
    //                             t.scan.insert(Vector2::new(x, l.y), Type::Water);
    //                         }
    //                         // todo.push(p)
    //                     }
    //                     (Hole(l), Wall(_)) => todo.push(l),
    //                     (Wall(_), Hole(r)) => {
    //                         todo.push(r);
    //                     }
    //                     (Hole(l), Hole(r)) => {
    //                         todo.push(l);
    //                         todo.push(r)
    //                     }
    //                 },
    //             }
    //         }
    //     }
    // }
}
fn count_wet(t: &T) -> usize {
    t.scan
        .iter()
        .filter(|&(pos, ty)| {
            t.upper_left.y <= pos.y
                && pos.y <= t.bottom_right.y
                && matches!(ty, Type::Water | Type::Transient)
        })
        .count()
}

fn count_dry(t: &T) -> usize {
    t.scan
        .iter()
        .filter(|&(pos, ty)| {
            t.upper_left.y <= pos.y && pos.y <= t.bottom_right.y && matches!(ty, Type::Water)
        })
        .count()
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut t = parse(&contents);
    pour(&mut t, Vector2::new(500, 0));
    println!("{}", count_wet(&t));
    println!("{}", count_dry(&t));
    if false {
        print(&t)
    };
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
        pour(&mut t, Vector2::new(500, 0));
        assert_eq!(count_wet(&t), 57)
    }
}
