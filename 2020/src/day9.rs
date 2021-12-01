use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_two_sum(v: &[u64], n: u64) -> bool {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if v[i] + v[j] == n {
                return true;
            }
        }
    }
    return false;
}

fn find(v: &[u64], n: usize) -> Option<(u64, u64)> {
    for i in n..v.len() {
        if !is_two_sum(&v[i - n..i], v[i]) {
            return Some((i as u64, v[i]));
        }
    }
    return None;
}

fn find_range_summing_to(v: &[u64], n: u64) -> Option<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    loop {
        assert!(i <= j);
        if v.len() <= j {
            return None;
        };
        if sum < n {
            sum += v[j];
            j += 1
        } else if sum > n {
            sum -= v[i];
            i += 1;
        } else {
            return Some((i, j));
        }
    }
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let num: u64 = line.parse().unwrap();
        input.push(num)
    }

    let (_index, n) = find(&input, 25).unwrap();
    println!("{}", n);

    let (i, j) = find_range_summing_to(&input, n).unwrap();
    let min = input[i..j].iter().min().unwrap();
    let max = input[i..j].iter().max().unwrap();
    println!("{} + {} = {}", min, max, min + max)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let v = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find(&v, 5), Some((14, 127)));
        assert_eq!(find_range_summing_to(&v, 127), Some((2, 6)));
        let min: u64 = *v[2..6].iter().min().unwrap();
        let max: u64 = *v[2..6].iter().max().unwrap();
        assert_eq!(min, 15);
        assert_eq!(max, 47);
    }
}
