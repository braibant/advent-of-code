use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq)]
pub enum T {
    F, // Floor
    O, // Occupied
    E, // Empty
}

fn parse(s: &str) -> Vec<T> {
    let mut row = Vec::new();
    for c in s.chars() {
        if c == '.' {
            row.push(T::F)
        } else if c == 'L' {
            row.push(T::E)
        } else {
            panic!("Unexpected input {}", s)
        }
    }
    return row;
}

fn count(grid: &Vec<Vec<T>>) -> u32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == T::O {
                count += 1
            }
        }
    }
    return count;
}

fn print(grid: &Vec<Vec<T>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == T::O {
                print!("#")
            } else if grid[i][j] == T::E {
                print!("L")
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}

fn step<F>(grid: &mut Vec<Vec<T>>, count: F, bound: u32) -> bool
where
    F: Fn(&Vec<Vec<T>>, i32, i32) -> u32,
{
    let mut counts = Vec::new();
    for i in 0..grid.len() {
        let mut line = Vec::new();
        for j in 0..grid[i].len() {
            line.push(count(grid, i as i32, j as i32));
        }
        counts.push(line);
    }

    let mut changes = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                T::O => {
                    if bound <= counts[i][j] {
                        grid[i][j] = T::E;
                        changes += 1
                    }
                }
                T::E => {
                    if counts[i][j] == 0 {
                        grid[i][j] = T::O;
                        changes += 1
                    }
                }
                _ => {}
            }
        }
    }
    return changes > 0;
}

mod part1 {
    use super::*;
    fn get(grid: &Vec<Vec<T>>, i: i32, j: i32) -> Option<T> {
        if 0 <= i && i < grid.len() as i32 {
            let row = &grid[i as usize];
            if 0 <= j && j < row.len() as i32 {
                return Some(row[j as usize]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn count(grid: &Vec<Vec<T>>, i: i32, j: i32) -> u32 {
        let mut n = 0;
        for i1 in i - 1..i + 2 {
            for j1 in j - 1..j + 2 {
                if i1 != i || j1 != j {
                    match get(grid, i1, j1) {
                        Some(T::O) => n += 1,
                        _ => {}
                    }
                }
            }
        }
        return n;
    }
}

mod part2 {
    use super::*;

    // Get, performing ray tracing along the vector di,dj
    fn get(grid: &Vec<Vec<T>>, i: i32, j: i32, di: i32, dj: i32) -> bool {
        let i = i + di;
        let j = j + dj;
        if 0 <= i && i < grid.len() as i32 {
            let row = &grid[i as usize];
            if 0 <= j && j < row.len() as i32 {
                let c = row[j as usize];
                match c {
                    T::O => true,
                    T::E => false,
                    T::F => get(grid, i, j, di, dj),
                }
            } else {
                false
            }
        } else {
            return false;
        }
    }

    pub fn count(grid: &Vec<Vec<T>>, i: i32, j: i32) -> u32 {
        let mut n = 0;
        for di in -1..2 {
            for dj in -1..2 {
                if di != 0 || dj != 0 {
                    if get(grid, i, j, di, dj) {
                        n += 1
                    }
                }
            }
        }
        return n;
    }
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        grid.push(parse(&line))
    }
    let mut grid1 = grid.clone();
    while {
        let changed = step(&mut grid1, part1::count, 4);
        changed
    } {}

    println!("{}", count(&grid1));

    let mut grid2 = grid.clone();
    while {
        let changed = step(&mut grid2, part2::count, 5);
        changed
    } {}

    println!("{}", count(&grid2))
}
