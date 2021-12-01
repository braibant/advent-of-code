use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
 // Iterating through OPS gives us all orientations
    static ref DIRS: Vec<(&'static str, i32, i32)> = {
        let dirs = vec![
            ("e", 2, 0),
            ("ne", 1, -1),
            ("nw", -1, -1),
            ("w", -2, 0),
            ("sw", -1, 1),
            ("se", 1, 1),
    ];

    return dirs};
}

fn parse(mut s: &str) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    'outer: while s.len() != 0 {
        for (d, dx, dy) in DIRS.iter() {
            if s.starts_with(d) {
                x += dx;
                y += dy;
                s = s.strip_prefix(d).unwrap();
                continue 'outer;
            }
        }
        panic!("Could not find matching prefix {}", s)
    }
    return (x, y);
}

fn step(state: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut neighbours: HashSet<(i32, i32)> = HashSet::new();

    for (x, y) in state.iter() {
        neighbours.insert((*x, *y));
        for (_, dx, dy) in DIRS.iter() {
            neighbours.insert((x + dx, y + dy));
        }
    }

    let mut next = HashSet::new();

    for (x, y) in neighbours.into_iter() {
        let mut adjacent_black_tiles = 0;
        for (_, dx, dy) in DIRS.iter() {
            if state.contains(&(x + dx, y + dy)) {
                adjacent_black_tiles += 1
            }
        }

        if state.contains(&(x, y)) && (adjacent_black_tiles == 1 || adjacent_black_tiles == 2) {
            next.insert((x, y));
        }
        if !state.contains(&(x, y)) && adjacent_black_tiles == 2 {
            next.insert((x, y));
        }
    }

    return next;
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut black = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let pos = parse(&line);

        if black.contains(&pos) {
            black.remove(&pos);
        } else {
            black.insert(pos);
        }
    }

    // part 1
    println!("{}", black.len());

    // part 2
    for i in 0..100 {
        black = step(&black);
    }
    println!("{}", black.len())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parsing() {
        let s = "sese";
        assert_eq!(parse(s), (2, 2));

        let s = "senw";
        assert_eq!(parse(s), (0, 0));

        let s = "nwwswee";
        assert_eq!(parse(s), (0, 0));
    }
}
