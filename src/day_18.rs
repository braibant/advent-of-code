// Puzzle input is a 2d map with some features (entrace denoted by @, keys
// denoted by lowercase letters, doors denoted by upper case letters). The goal
// is to find the shortest path that collects all keys.
// 0. Pick coordinate system ((x,y) with ---> x and v y )
// 1. Parse the input (typical map structure as sets of points).
// 2. Compute shortest paths between points of interest (key, doors, entrance),
//    which changes with the set of keys that has been collected. The examples
//    demonstrate that the greedy solution (picking the closest key first) is
//    not optimal.
// 3. Maybe, we can compute the distance between two keys (or the entrance and a
//    key) as the tuple of the number of steps, and the set of doors to
//    traverse,

use crate::vector2::Vector2;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

lazy_static! {
    // N, E, S, W. (0,0) is top left of screen.
    static ref DIRS: [Vector2<i64>; 4] = [Vector2::new(0, -1), Vector2::new(1, 0), Vector2::new(0, 1), Vector2::new(-1, 0)];
}

#[derive(Clone)]
struct T {
    doors: HashMap<Vector2<i64>, u8>,
    keys: HashMap<Vector2<i64>, u8>,
    entrance: Vec<Vector2<i64>>,
    tiles: HashSet<Vector2<i64>>,
}

impl T {
    fn new() -> T {
        T {
            doors: HashMap::new(),
            keys: HashMap::new(),
            entrance: vec![Vector2::new(0, 0)],
            tiles: HashSet::new(),
        }
    }
    fn key_position(&self, key: u8) -> Option<Vector2<i64>> {
        for (&pos, &k) in self.keys.iter() {
            if k == key {
                return Some(pos);
            }
        }
        return None;
    }
}

fn neighbours(t: &T, p: Vector2<i64>) -> Vec<Vector2<i64>> {
    DIRS.iter()
        .map(|&d| p + d)
        .filter(|n| t.tiles.contains(&n))
        .collect()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Frame {
    pos: Vec<Vector2<i64>>,
    keys: Vec<u8>,
}

impl Frame {
    fn set_position(&self, robot: usize, pos: Vector2<i64>) -> Frame {
        let mut frame = self.clone();
        frame.pos[robot] = pos;
        frame
    }

    fn add_key(&self, key: u8) -> Frame {
        let mut frame = self.clone();
        assert!(frame.keys.iter().all(|&k| k != key));
        frame.keys.push(key);
        frame.keys.sort();
        frame
    }
}

// BFS of the set of reachable keys given the current state. Return a list of moves [key, cost]
fn reachable_keys(t: &T, frame: &Frame, robot: usize) -> Vec<(u8, u32)> {
    let mut queue: VecDeque<(Vector2<i64>, u32)> = std::collections::VecDeque::new();
    let mut reachable = HashMap::new();
    let mut visited = HashSet::new();
    queue.push_back((frame.pos[robot], 0));
    while !queue.is_empty() {
        let (pos, distance) = queue.pop_front().unwrap();
        if visited.contains(&pos) {
        } else {
            visited.insert(pos);
            for next in neighbours(t, pos).iter() {
                if let Some(key) = t.keys.get(next) {
                    if reachable.contains_key(key) {
                    } else if frame.keys.contains(key) {
                        queue.push_back((*next, distance + 1))
                    } else {
                        reachable.insert(key, distance + 1);
                        // We stop here, to handover to the Dijkstra part of the
                        // search.
                    }
                } else if let Some(door) = t.doors.get(next) {
                    if frame.keys.contains(door) {
                        // This is a door we can open
                        queue.push_back((*next, distance + 1));
                    }
                } else {
                    // This is a regular tile
                    queue.push_back((*next, distance + 1))
                }
            }
        }
    }
    reachable.iter().map(|(&k, &v)| (*k, v)).collect()
}

fn dijkstra(t: &T) -> Option<u32> {
    let mut cost: HashMap<Frame, _> = HashMap::new(); // Absence from the map means cost = + \infnty
    let mut queue: VecDeque<Frame> = std::collections::VecDeque::new();
    let init_frame = Frame {
        pos: t.entrance.clone(),
        keys: vec![],
    };
    queue.push_back(init_frame.clone());
    cost.insert(init_frame, 0);
    while !queue.is_empty() {
        let frame = queue.pop_front().unwrap();
        let children: Vec<_> = (0..frame.pos.len())
            .flat_map(|i| {
                reachable_keys(t, &frame, i)
                    .iter()
                    .map(|(pos, cost)| (i, *pos, *cost))
                    .collect::<Vec<_>>()
            })
            .collect();
        for (robot, k, add) in children.iter() {
            let pos = t.key_position(*k).unwrap();
            let next_frame = frame.add_key(*k).set_position(*robot, pos);
            let &c0 = cost.get(&frame).unwrap();
            let update = match cost.get(&next_frame) {
                Some(&c1) => c1 > c0 + add,
                None => true,
            };
            if update {
                queue.push_back(next_frame.clone());
                cost.insert(next_frame, c0 + add);
            }
        }
    }
    // println!(
    //     "{}",
    //     cost.iter()
    //         .filter(|(frame, cost)| frame.keys.len() == t.keys.len())
    //         .count()
    // );
    cost.iter()
        .filter_map(|(frame, cost)| {
            if frame.keys.len() == t.keys.len() {
                Some(*cost)
            } else {
                None
            }
        })
        .min()
}

fn parse(content: &str) -> T {
    let mut t = T::new();
    for (y, line) in content.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Vector2::new(x as i64, y as i64);
            if c.is_ascii_lowercase() {
                let key = (c as u8) - ('a' as u8);
                t.keys.insert(pos, key);
                t.tiles.insert(pos);
            } else if c.is_ascii_uppercase() {
                let door = (c as u8) - ('A' as u8);
                t.doors.insert(pos, door);
                t.tiles.insert(pos);
            } else if c == '@' {
                t.entrance[0] = pos;
                t.tiles.insert(pos);
            } else if c == '.' {
                t.tiles.insert(pos);
            } else if c == '#' {
                // wall
            } else {
                panic!("Unecpected input {}", c)
            }
        }
    }
    t
}

fn split_vault(t: &T) -> T {
    assert!(t.entrance.len() == 1);
    let mut t = t.clone();
    let entrance = t.entrance[0];
    t.entrance.clear();

    // Add the 4 new vault entrance
    t.entrance.push(entrance + Vector2::new(-1, -1));
    t.entrance.push(entrance + Vector2::new(1, -1));
    t.entrance.push(entrance + Vector2::new(-1, 1));
    t.entrance.push(entrance + Vector2::new(1, 1));

    // Add new walls
    t.tiles.remove(&(entrance + Vector2::new(1, 0)));
    t.tiles.remove(&(entrance + Vector2::new(-1, 0)));
    t.tiles.remove(&(entrance + Vector2::new(0, 1)));
    t.tiles.remove(&(entrance + Vector2::new(0, -1)));
    t
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let t = parse(&content);
    println!("{:?}", dijkstra(&t));

    // part 2
    let t = split_vault(&t);
    println!("{:?}", dijkstra(&t));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {
        let s = "#########
#b.A.@.a#
#########";
        let t = parse(s);
        assert_eq!(dijkstra(&t), Some(8))
    }
    #[test]
    fn test_case_2() {
        let s = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        let t = parse(s);
        assert_eq!(dijkstra(&t), Some(132))
    }

    #[test]
    #[ignore]
    fn test_case_3() {
        let s = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        let t = parse(s);
        assert_eq!(dijkstra(&t), Some(136))
    }

    #[test]
    fn test_case_4() {
        let s = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        let t = parse(s);
        assert_eq!(dijkstra(&t), Some(81))
    }
}
