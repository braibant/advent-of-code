use std::fs::File;
use std::io::{BufRead, BufReader};

// Assumes that [v] is sorted
fn sum2(v: &[i32], tgt: i32) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        if v[i] + v[j] < tgt {
            i += 1
        } else if v[i] + v[j] > tgt {
            j -= 1
        } else {
            assert_eq!(v[i] + v[j], tgt);
            return Some((i, j));
        }
    }
    return None;
}

// Assumes that [v] is sorted
fn sum3(v: &[i32], tgt: i32) -> Option<(usize, usize, usize)> {
    for i in 0..v.len() {
        match sum2(&v[i + 1..], tgt - v[i]) {
            None => (),
            Some((j, k)) => {
                return Some((i, j + i + 1, k + i + 1));
            }
        }
    }
    return None;
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num: i32 = line.parse().unwrap();
        content.push(num);
    }

    content.sort();
    let (i, j) = sum2(&content, 2020).unwrap();
    println!("sum2:{}", content[i] * content[j]);

    let (i, j, k) = sum3(&content, 2020).unwrap();
    println!(
        "sum3:{} {} {} {}",
        content[i],
        content[j],
        content[k],
        content[i] * content[j] * content[k]
    );
}
