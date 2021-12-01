use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};
struct T {
    depth: i32,
    target_x: i32,
    target_y: i32,
    memo_erosion_level: HashMap<(i32, i32), i32>,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
struct N {
    x: i32,
    y: i32,
    torch: bool,
    climbing_gear: bool,
}

impl T {
    fn geologic_index(&mut self, x: i32, y: i32) -> i32 {
        if (x == 0 && y == 0) || (x == self.target_x && y == self.target_y) {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1)
        }
    }

    fn erosion_level(&mut self, x: i32, y: i32) -> i32 {
        if self.memo_erosion_level.contains_key(&(x, y)) {
            *self.memo_erosion_level.get(&(x, y)).unwrap()
        } else {
            let r = (self.geologic_index(x, y) + self.depth) % 20183;
            self.memo_erosion_level.insert((x, y), r);
            r
        }
    }

    // type: 0 rocky, 1 wet, 2 narrow
    // risk level: 0 for rocky, 1 for wet, 2 for narrow
    fn soil(&mut self, x: i32, y: i32) -> i32 {
        self.erosion_level(x, y) % 3
    }

    fn is_valid(&mut self, n: &N) -> bool {
        let soil = self.soil(n.x, n.y);
        (soil == 0 && (n.climbing_gear || n.torch))
            || (soil == 1 && (!n.torch))
            || (soil == 2 && (!n.climbing_gear))
    }

    fn neighbors(&mut self, n: N) -> Vec<(i64, N)> {
        let mut acc = Vec::new();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = n.x + dx;
            let y = n.y + dy;

            if x < 0 || y < 0 {
                break;
            }

            let m = N { x, y, ..n };
            if self.is_valid(&m) {
                acc.push((1, m));
            }
        }

        for torch in [true, false] {
            for climbing_gear in [true, false] {
                if (!climbing_gear || !torch)
                    && (torch != n.torch || climbing_gear != n.climbing_gear)
                {
                    acc.push((
                        7,
                        N {
                            torch,
                            climbing_gear,
                            ..n
                        },
                    ))
                }
            }
        }

        acc.into_iter().filter(|(_, n)| self.is_valid(n)).collect()
    }
}

fn risk_level(t: &mut T) -> i64 {
    let mut level = 0i64;
    for x in 0..=t.target_x {
        for y in 0..=t.target_y {
            level += t.soil(x, y) as i64;
        }
    }

    level
}

// Part 2 boils down to computing the path of minimum weight in the graph where
// nodes are a pair of a position and a tool, and edges are transitionning tools
// or moving positions.
fn distance(t: &mut T) -> i64 {
    let mut distance = HashMap::new();
    let mut heap = BinaryHeap::new();

    let source = N {
        x: 0,
        y: 0,
        torch: true,
        climbing_gear: false,
    };

    use std::cmp::Reverse;

    heap.push(Reverse((0, source)));
    distance.insert(source, 0);

    while let Some(Reverse((time, u))) = heap.pop() {
        if u.x == t.target_x && u.y == t.target_y && u.torch {
            return time;
        };

        // We have found a better way
        if time > distance[&u] {
            continue;
        };

        for (dt, v) in t.neighbors(u) {
            if time + dt < *distance.get(&v).unwrap_or(&i64::MAX) {
                heap.push(Reverse((time + dt, v)));
                distance.insert(v, time + dt);
            }
        }
    }
    panic!()
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<_> = contents.lines().collect();
    let depth = lines[0].strip_prefix("depth: ").unwrap().parse().unwrap();
    let target_pos: Vec<_> = lines[1]
        .strip_prefix("target: ")
        .unwrap()
        .split(',')
        .collect();
    let target_x = target_pos[0].parse().unwrap();
    let target_y = target_pos[1].parse().unwrap();
    let mut t = T {
        depth,
        target_x,
        target_y,
        memo_erosion_level: HashMap::new(),
    };
    println!("{}", risk_level(&mut t));
    println!("{}", distance(&mut t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut E = T {
            depth: 510,
            target_x: 10,
            target_y: 10,
            memo_erosion_level: HashMap::new(),
        };
        assert_eq!(E.geologic_index(0, 0), 0);
        assert_eq!(E.erosion_level(0, 0), 510);

        assert_eq!(E.erosion_level(1, 0), 17317);

        assert_eq!(E.erosion_level(0, 1), 8415);

        assert_eq!(E.erosion_level(1, 1), 1805);

        assert_eq!(E.erosion_level(10, 10), 510);
    }

    #[test]
    fn part2() {
        let mut E = T {
            depth: 510,
            target_x: 10,
            target_y: 10,
            memo_erosion_level: HashMap::new(),
        };
        assert_eq!(distance(&mut E), 45);
    }
}
