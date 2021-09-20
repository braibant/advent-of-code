use std::collections::HashMap;

struct Scan {
    twice: bool,
    thrice: bool,
}

fn scan_id(id: &str) -> Scan {
    let mut counts = HashMap::new();
    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut twice = false;
    let mut thrice = false;
    for (_k, &v) in counts.iter() {
        if v == 2 {
            twice = true
        };
        if v == 3 {
            thrice = true
        }
    }
    Scan {
        twice: twice,
        thrice: thrice,
    }
}

fn part1(s: &str) -> i64 {
    let mut twice = 0;
    let mut thrice = 0;
    for id in s.split('\n') {
        let scan = scan_id(id);
        if scan.twice {
            twice += 1
        };
        if scan.thrice {
            thrice += 1
        }
    }
    twice * thrice
}

fn d1(a: &str, b: &str) -> bool {
    let mut distance = 0;
    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            distance += 1
        };
    }
    distance == 1
}

fn common(a: &str, b: &str) -> String {
    let mut buf = String::new();
    for (a, b) in a.chars().zip(b.chars()) {
        if a == b {
            buf.push(a)
        };
    }
    buf
}

fn part2(s: &str) -> String {
    let ids: Vec<&str> = s.split('\n').collect();
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            if d1(ids[i], ids[j]) {
                return common(&ids[i], &ids[j]);
            }
        }
    }
    panic!("Invalid input")
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    println!("{}", part1(&content));
    println!("{}", part2(&content));
}
