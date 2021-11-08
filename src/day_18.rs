use crate::util::vector2::Vector2;
use std::collections::HashMap;
use std::hash::Hash;
type Position = Vector2<i32>;

const N: i32 = 50;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct T {
    grid: Vec<Vec<Tile>>,
    size: i32,
}

struct Adj {
    trees: usize,
    lumberyards: usize,
    open: usize,
}

impl T {
    fn get(&self, pos: Position) -> Option<Tile> {
        if 0 <= pos.x && pos.x < self.size && 0 <= pos.y && pos.y < self.size {
            Some(self.grid[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    fn adjacent(&self, pos: Position) -> Adj {
        let mut adj = Adj {
            trees: 0,
            lumberyards: 0,
            open: 0,
        };
        for dx in -1..=1_i32 {
            for dy in -1..=1_i32 {
                if dx != 0 || dy != 0 {
                    match self.get((pos + Vector2::new(dx, dy))) {
                        None => {}
                        Some(Tile::Lumberyard) => adj.lumberyards += 1,
                        Some(Tile::Trees) => adj.trees += 1,
                        Some(Tile::Open) => adj.open += 1,
                    }
                }
            }
        }
        adj
    }

    fn iter(&self) -> IterT<'_> {
        IterT {
            what: self,
            x: 0,
            y: 0,
            done: false,
        }
    }
}

struct IterT<'a> {
    what: &'a T,
    x: i32,
    y: i32,
    done: bool,
}

impl<'a> Iterator for IterT<'a> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let r = self.what.grid[self.y as usize][self.x as usize];
            if self.x < self.what.size - 1 {
                self.x += 1
            } else if self.y < self.what.size - 1 {
                self.x = 0;
                self.y += 1;
            } else {
                self.done = true;
            }
            Some(r)
        }
    }
}

fn step(t: &T) -> T {
    let mut next = Vec::new();
    for y in 0..t.size {
        let mut line = vec![];
        for x in 0..t.size {
            let pos = Vector2::new(x, y);
            let adjacent = t.adjacent(pos);
            let tile = match t.get(pos).unwrap() {
                Tile::Open => {
                    if adjacent.trees >= 3 {
                        Tile::Trees
                    } else {
                        Tile::Open
                    }
                }
                Tile::Trees => {
                    if adjacent.lumberyards >= 3 {
                        Tile::Lumberyard
                    } else {
                        Tile::Trees
                    }
                }
                Tile::Lumberyard => {
                    if adjacent.trees >= 1 && adjacent.lumberyards >= 1 {
                        Tile::Lumberyard
                    } else {
                        Tile::Open
                    }
                }
            };
            line.push(tile)
        }
        next.push(line);
    }
    T {
        grid: next,
        size: t.size,
    }
}

fn part1(mut t: T) -> (usize, usize) {
    for _i in 0..10 {
        t = step(&t);
        // print_grid(
        //     tile_to_char,
        //     &t.grid,
        //     Vector2::new(0, 0),
        //     Vector2::new(t.size - 1, t.size - 1),
        // )
    }

    let trees = t.iter().filter(|t| matches!(t, Tile::Trees)).count();
    let lumberyards = t.iter().filter(|t| matches!(t, Tile::Lumberyard)).count();
    (trees, lumberyards)
}

fn part2(mut t: T) -> usize {
    let mut visited = HashMap::new();

    let mut time = 0;
    while visited.get(&t).is_none() {
        visited.insert(t.clone(), time);
        time += 1;
        t = step(&t);
    }
    let t0 = visited.get(&t).unwrap();
    let period = time - t0;
    let target = 1000000000;
    let remainder = (target - t0) % period;

    for _i in 0..remainder {
        t = step(&t)
    }
    let trees = t.iter().filter(|t| matches!(t, Tile::Trees)).count();
    let lumberyards = t.iter().filter(|t| matches!(t, Tile::Lumberyard)).count();
    trees * lumberyards
}

fn parse(s: &str, size: i32) -> T {
    let mut grid = vec![];
    for line in s.lines() {
        let mut l = vec![];
        for c in line.chars() {
            let t = match c {
                '.' => Tile::Open,
                '|' => Tile::Trees,
                '#' => Tile::Lumberyard,
                _ => panic!("Unexpected input {}", c),
            };
            l.push(t);
        }
        grid.push(l)
    }

    T { grid, size }
}
pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let t = parse(&contents, N);
    let (trees, lumberyards) = part1(t.clone());

    println!("{} * {} = {}", trees, lumberyards, trees * lumberyards);
    println!("{}", part2(t.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn test_sample() {
        let t = parse(SAMPLE, 10);
        assert_eq!(part1(t), (37, 31))
    }
}

// fn grid(s: &str) -> HashMap<Position, char> {
//     s.lines()
//         .enumerate()
//         .flat_map(|(y, l)| {
//             l.chars()
//                 .enumerate()
//                 .map(move |(x, c)| (Vector2::new(x as i32, y as i32), c))
//         })
//         .collect()
// }

// #[allow(dead_code)]
// fn print_grid<T, F>(f: F, t: &HashMap<Position, T>, ul: Position, br: Position)
// where
//     F: Fn(&T) -> char,
// {
//     let mut s = String::new();
//     for y in ul.y..=br.y {
//         for x in ul.x..=br.y {
//             match t.get(&Vector2::new(x, y)) {
//                 None => s.push(' '),
//                 Some(t) => s.push(f(t)),
//             }
//         }
//         s.push('\n')
//     }
//     println!("{}\n", s);
// }

// #[allow(dead_code)]
// fn tile_to_char(tile: &Tile) -> char {
//     match tile {
//         Tile::Open => '.',
//         Tile::Trees => '|',
//         &Tile::Lumberyard => '#',
//     }
// }

//    let grid = grid(s)
//         .into_iter()
//         .map(|(p, c)| {
//             let t = match c {
//                 '.' => Tile::Open,
//                 '|' => Tile::Trees,
//                 '#' => Tile::Lumberyard,
//                 _ => panic!("Unexpected input {}", c),
//             };
//             (p, t)
//         })
//         .collect();
