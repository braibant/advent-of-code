use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    static ref DIRS3: Vec<(i32, i32, i32, i32)> = {
        let mut dirs = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    if i != 0 || j != 0 || k != 0 {
                        dirs.push((i, j, k, 0))
                    }
                }
            }
        }
        dirs
    };
    static ref DIRS4: Vec<(i32, i32, i32, i32)> = {
        let mut dirs = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    for l in -1..2 {
                        if i != 0 || j != 0 || k != 0 || l != 0 {
                            dirs.push((i, j, k, l))
                        }
                    }
                }
            }
        }
        dirs
    };
}

fn count(
    cubes: &HashSet<(i32, i32, i32, i32)>,
    dirs: &Vec<(i32, i32, i32, i32)>,
    x: i32,
    y: i32,
    z: i32,
    w: i32,
) -> u32 {
    let mut count = 0;
    for (dx, dy, dz, dw) in dirs.iter() {
        if cubes.contains(&(x + dx, y + dy, z + dz, w + dw)) {
            count += 1
        }
    }
    return count;
}

fn step(
    cubes: &HashSet<(i32, i32, i32, i32)>,
    dirs: &Vec<(i32, i32, i32, i32)>,
) -> HashSet<(i32, i32, i32, i32)> {
    let mut neighbours = HashSet::new();

    for (x, y, z, w) in cubes.iter() {
        for (dx, dy, dz, dw) in dirs.iter() {
            neighbours.insert((x + dx, y + dy, z + dz, w + dw));
        }
    }

    let mut next: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    for &(x, y, z, w) in neighbours.iter() {
        let count = count(cubes, dirs, x, y, z, w);
        let active = cubes.contains(&(x, y, z, w));
        if active && (count == 2 || count == 3) {
            next.insert((x, y, z, w));
        };
        if !active && count == 3 {
            next.insert((x, y, z, w));
        };
    }
    return next;
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut init: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                init.insert((i as i32, j as i32, 0, 0));
            } else if char == '.' {
            } else {
                panic!("Invalid input {}", line)
            }
        }
    }

    let mut cubes = init.clone();
    for i in 0..6 {
        cubes = step(&cubes, &DIRS3);
    }

    println!("{}", cubes.len());

    let mut cubes = init.clone();
    for i in 0..6 {
        cubes = step(&cubes, &DIRS4);
    }

    println!("{}", cubes.len());
}
