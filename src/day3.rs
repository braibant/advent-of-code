use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_slope(map: &Vec<Vec<u8>>, right: usize, down: usize) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut n = 0;
    let len = map[0].len();
    while i < map.len() {
        if map[i][j % len] == b'#' {
            n += 1
        };
        i += down;
        j += right
    }
    return n;
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<u8>> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line = line.as_bytes();
        map.push(line.to_vec())
    }

    let s11 = check_slope(&map, 1, 1);
    let s31 = check_slope(&map, 3, 1);
    let s51 = check_slope(&map, 5, 1);
    let s71 = check_slope(&map, 7, 1);
    let s12 = check_slope(&map, 1, 2);
    println!("{}", s31);
    println!("{}", s11 * s31 * s51 * s71 * s12);
}
