use lazy_static::lazy_static;
use std::collections::HashSet;
const N: i8 = 5;
lazy_static! {
    static ref DIRS2: Vec<(i8, i8)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
}

fn parse(s: &str) -> Vec<(i8, i8)> {
    let mut result = Vec::new();
    for (i, line) in s.split('\n').enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                result.push((i as i8, j as i8))
            }
        }
    }
    result
}

trait Positions<'a> {
    type Position;
    fn new() -> Self;
    fn get(&self, pos: &Self::Position) -> bool;
    fn add(&mut self, pos: Self::Position);
    fn neighbours(&self, pos: &Self::Position) -> Vec<Self::Position>;
    fn positions(&self) -> Box<dyn Iterator<Item = Self::Position> + '_>;
    fn len(&self) -> usize;
}

impl<'a> Positions<'a> for u64 {
    type Position = (i8, i8);

    fn new() -> Self {
        0
    }

    fn len(&self) -> usize {
        let mut count = 0;
        for i in 0..N {
            for j in 0..N {
                if self.get(&(i, j)) {
                    count += 1
                }
            }
        }
        count
    }

    fn get(&self, pos: &Self::Position) -> bool {
        let (i, j) = *pos;
        if i < 0 || N <= i || j < 0 || N <= j {
            false
        } else {
            (*self & (1 << (i * N + j))) != 0
        }
    }

    fn add(&mut self, pos: Self::Position) {
        let (i, j) = pos;
        if i < 0 || N <= i || j < 0 || N <= j {
            panic!("Invalid bitset operation {} {}", i, j)
        } else {
            let mask = 1 << (i * N + j);
            *self |= mask
        }
    }

    fn neighbours(&self, pos: &Self::Position) -> Vec<Self::Position> {
        let mut result = Vec::new();
        let (i, j) = *pos;
        for (di, dj) in DIRS2.iter() {
            let i = i + di;
            let j = j + dj;
            if 0 <= i && i < N && 0 <= j && j < N {
                result.push((i, j))
            }
        }
        result
    }
    fn positions(&self) -> Box<dyn Iterator<Item = Self::Position> + '_> {
        let mut result: Vec<(i8, i8)> = Vec::new();
        for i in 0..N {
            for j in 0..N {
                result.push((i, j))
            }
        }
        Box::new(result.into_iter())
    }
}

fn biodiversity_rating(s: u64) -> u64 {
    let mut result = 0;
    for i in 0..N {
        for j in 0..N {
            if s.get(&(i, j)) {
                result += 1 << (i * N + j)
            }
        }
    }
    result
}

impl<'a> Positions<'a> for HashSet<(i8, i8, i64)> {
    type Position = (i8, i8, i64);

    fn positions(&self) -> Box<dyn Iterator<Item = Self::Position> + '_> {
        Box::new(self.iter().copied())
    }

    fn new() -> Self {
        HashSet::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, pos: &Self::Position) -> bool {
        self.contains(pos)
    }

    fn add(&mut self, pos: Self::Position) {
        self.insert(pos);
    }

    fn neighbours(&self, pos: &Self::Position) -> Vec<Self::Position> {
        // There are many cases to consider. If we are on the outer border of
        // the grid at a given level, we need to consider the extra neighbours
        // at level -1. If we are on the next to the center, we need to consider
        // the extra neighbours at level + 1.
        let mut result = Vec::new();
        let mut push = |i: i8, j: i8, l: i64| {
            if i == 2 && j == 2 {
            } else {
                result.push((i, j, l))
            }
        };
        let (i, j, level) = *pos;
        if i == 0 {
            push(1, 2, level - 1)
        };
        if i == N - 1 {
            push(3, 2, level - 1)
        };
        if j == 0 {
            push(2, 1, level - 1)
        };
        if j == N - 1 {
            push(2, 3, level - 1)
        };
        if i == 1 && j == 2 {
            // Add top row
            for x in 0..N {
                push(0, x, level + 1)
            }
        };
        if i == 3 && j == 2 {
            // Add bottom row
            for x in 0..N {
                push(N - 1, x, level + 1)
            }
        };
        if i == 2 && j == 1 {
            // Add the left row
            for x in 0..N {
                push(x, 0, level + 1)
            }
        };
        if i == 2 && j == 3 {
            // Add the right row
            for x in 0..N {
                push(x, N - 1, level + 1)
            }
        };
        if i + 1 < N {
            push(i + 1, j, level)
        };
        if 0 <= i - 1 {
            push(i - 1, j, level)
        };
        if j + 1 < N {
            push(i, j + 1, level)
        };
        if 0 <= j - 1 {
            push(i, j - 1, level)
        };

        result
    }
}

fn count<'a, T>(state: &T, pos: T::Position) -> usize
where
    T: Positions<'a>,
    T::Position: Eq,
{
    state
        .neighbours(&pos)
        .iter()
        .filter(|n| state.get(n))
        .count()
}

fn next<'a, T>(state: &T) -> T
where
    T: Positions<'a>,
    T::Position: Eq + std::hash::Hash + Copy,
{
    let mut to_consider = HashSet::new();
    for pos in state.positions() {
        to_consider.insert(pos);
        to_consider.extend(state.neighbours(&pos))
    }

    let mut next = T::new();
    for &pos in to_consider.iter() {
        let n = count(state, pos);
        let infested = state.get(&pos);
        if n == 1 && infested {
            next.add(pos)
        } else if !infested && (n == 1 || n == 2) {
            next.add(pos)
        } else {
        }
    }
    next
}

fn part1(positions: &[(i8, i8)]) -> u64 {
    let mut state: u64 = 0;
    positions.iter().for_each(|(i, j)| state.add((*i, *j)));
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(state) {
            return (biodiversity_rating(state));
        }
        state = next(&state);
    }
}

fn part2(positions: &[(i8, i8)], steps: usize) -> usize {
    let mut state: HashSet<(i8, i8, i64)> = HashSet::new();
    positions.iter().for_each(|(i, j)| state.add((*i, *j, 0)));
    for _i in 0..steps {
        state = next(&state)
    }
    state.len()
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let positions = parse(&contents);
    println!("{}", part1(&positions));
    println!("{}", part2(&positions, 200));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        let s = "....#
#..#.
#..##
..#..
#....";
        let positions = parse(s);
        assert_eq!(part1(&positions), 2129920);
    }

    #[test]
    fn test_example2() {
        let s = "....#
#..#.
#..##
..#..
#....";
        let positions = parse(s);
        assert_eq!(part2(&positions, 10), 99);
    }

    #[test]
    fn test_neighbours() {
        let s = "....#
#..#.
#..##
..#..
#....";
        let positions = parse(s);
        let mut state: HashSet<(i8, i8, i64)> = HashSet::new();
        positions.iter().for_each(|(i, j)| state.add((*i, *j, 0)));
        assert_eq!(
            state.neighbours(&(0, 0, 0)),
            vec![(1, 2, -1), (2, 1, -1), (1, 0, 0), (0, 1, 0)]
        );
        assert_eq!(
            state.neighbours(&(2, 1, 0)),
            vec![
                (0, 0, 1),
                (1, 0, 1),
                (2, 0, 1),
                (3, 0, 1),
                (4, 0, 1),
                (3, 1, 0),
                (1, 1, 0),
                (2, 0, 0)
            ]
        );
        assert_eq!(
            state.neighbours(&(2, 3, 0)),
            vec![
                (0, 4, 1),
                (1, 4, 1),
                (2, 4, 1),
                (3, 4, 1),
                (4, 4, 1),
                (3, 3, 0),
                (1, 3, 0),
                (2, 4, 0)
            ]
        );
    }
}
