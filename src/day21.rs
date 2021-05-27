use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let _reader = BufReader::new(file);
}
