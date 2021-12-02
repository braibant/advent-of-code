use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

struct UnionFind<T> {
    parent: HashMap<T, T>,
    rank: HashMap<T, u32>,
}

impl<T> UnionFind<T>
where
    T: std::hash::Hash + std::cmp::Eq + Clone,
{
    fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn add(&mut self, x: T) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x.clone(), x.clone());
            self.rank.insert(x.clone(), 0);
        }
    }

    fn find(&mut self, x: T) -> T {
        let y = self.parent.get(&x).cloned().unwrap();

        if y != x {
            let z = self.find(y);
            self.parent.insert(x, z.clone());
            z
        } else {
            x
        }
    }

    fn merge(&mut self, x: T, y: T) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
        } else {
            let rx = self.rank.get(&x).unwrap();
            let ry = self.rank.get(&y).unwrap();
            let (x, y) = if rx < ry { (y, x) } else { (x, y) };
            self.parent.insert(y.clone(), x.clone());
            if rx == ry {
                self.rank.insert(x, rx + 1);
            }
        }
    }
}

fn parse(s: &str) -> Vec<Position> {
    s.lines()
        .map(|line| {
            let p: Vec<_> = line.split(',').map(|i| i.parse().unwrap()).collect();
            Position {
                x: p[0],
                y: p[1],
                z: p[2],
                t: p[3],
            }
        })
        .collect()
}

fn distance(a: &Position, b: &Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs() + (a.t - b.t).abs()
}

fn part1(positions: &[Position]) -> usize {
    let mut constellations: UnionFind<Position> = UnionFind::new();

    'outer: for i in 0..positions.len() {
        constellations.add(positions[i]);
        for j in 0..i {
            if distance(&positions[i], &positions[j]) <= 3 {
                constellations.merge(positions[i], positions[j]);
            }
        }
    }

    let mut classes = HashSet::new();
    for i in 0..positions.len() {
        let root = constellations.find(positions[i]);
        classes.insert(root);
    }
    classes.len()
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let points = parse(&content);
    println!("{}", part1(&points));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Position {
            x: 1,
            y: -1,
            z: 0,
            t: -1,
        };
        let p2 = Position {
            x: 2,
            y: -2,
            z: 0,
            t: -1,
        };
        assert_eq!(distance(&p1, &p2), 2)
    }

    const E1: &str = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";

    #[test]
    fn example1() {
        let positions = parse(E1);
        assert_eq!(part1(&positions), 4);
    }

    const E2: &str = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";

    #[test]
    fn example2() {
        let positions = parse(E2);
        assert_eq!(part1(&positions), 3);
    }

    const E3: &str = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";

    #[test]
    fn example3() {
        let positions = parse(E3);
        assert_eq!(part1(&positions), 8);
    }
}
