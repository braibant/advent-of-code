use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_arrangements(v: &[u32]) -> u64 {
    println!("{:?}", v);
    let delta = |i: usize, j: usize| -> u64 {
        if v[j] - v[i] <= 3 {
            return 1;
        } else {
            return 0;
        }
    };
    let mut arrangements: Vec<u64> = Vec::new();
    arrangements.resize(v.len(), 0);
    arrangements[0] = 1;
    arrangements[1] = delta(0, 1);
    arrangements[2] = delta(0, 2) + delta(0, 1) * delta(1, 2);
    for i in 3..v.len() {
        arrangements[i] = arrangements[i - 1] * delta(i - 1, i)
            + arrangements[i - 2] * delta(i - 2, i)
            + arrangements[i - 3] * delta(i - 3, i)
    }
    println!("{:?}", arrangements);
    return arrangements[v.len() - 1];
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut adapters: Vec<(u32)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let joltage: u32 = line.parse().unwrap();
        adapters.push(joltage);
    }
    adapters.push(0);
    let builtin: u32 = adapters.iter().max().unwrap() + 3;
    adapters.push(builtin);

    adapters.sort();
    // We know that there exist a solution to the problem using all adapters,
    // with differences between joltages less or equal than 3
    let mut diffs: [u32; 4] = [0; 4];
    for i in 1..adapters.len() {
        diffs[(adapters[i] - adapters[i - 1]) as usize] += 1;
    }
    println!("{}", diffs[1] * diffs[3]);
    println!("{}", count_arrangements(&adapters));
}
