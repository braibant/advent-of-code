use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

enum Problem {
    Day1(String),
    Day2(String),
}

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

fn day1(filename: String) {
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

#[derive(Debug)]
struct Password {
    low: i32,
    high: i32,
    letter: char,
    password: String,
}

impl Password {
    fn parse(s: String) -> Option<Password> {
        let v: Vec<&str> = s.split(&['-', ':', ' '][..]).collect();
        if v.len() != 5 {
            return None;
        } else {
            let low: i32 = v[0].parse().unwrap();
            let high: i32 = v[1].parse().unwrap();
            let letter: char = v[2].parse().unwrap();
            let password: String = v[4].parse().unwrap();
            Some(Password {
                low,
                high,
                letter,
                password,
            })
        }
    }

    fn check1(&self) -> bool {
        fn count(x: char, s: &str) -> i32 {
            let mut i = 0;
            for c in s.chars() {
                if c == x {
                    i += 1
                }
            }
            i
        }
        let n = count(self.letter, &self.password);
        self.low <= n && n <= self.high
    }

    fn check2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let p1 = chars[(self.low - 1) as usize] == self.letter;
        let p2 = chars[(self.high - 1) as usize] == self.letter;
        !(p1 && p2) && (p1 || p2)
    }
}

fn day2(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut check1 = 0;
    let mut check2 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let password = Password::parse(line).unwrap();
        if Password::check1(&password) {
            check1 += 1
        };
        if Password::check2(&password) {
            check2 += 1
        }
    }
    println!("check1 {}", check1);
    println!("check2 {}", check2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day2_example() {
        let password = Password::parse("1-3 a: abcde".to_string()).unwrap();
        assert!(Password::check(&password));
        let password = Password::parse("1-3 b: cdefg".to_string()).unwrap();
        assert!(!(Password::check(&password)))
    }
}
impl Problem {
    fn new(args: &[String]) -> Result<Problem, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        };
        let day = args[1].clone();
        match day.as_str() {
            "1" => {
                if args.len() < 3 {
                    return Err("not enough arguments");
                };
                return Ok(Problem::Day1(args[2].clone()));
            }
            "2" => {
                if args.len() < 3 {
                    return Err("not enough arguments");
                };
                return Ok(Problem::Day2(args[2].clone()));
            }
            _ => return Err("Problem not yet implemented"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = Problem::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    match problem {
        Problem::Day1(filename) => day1(filename),
        Problem::Day2(filename) => day2(filename),
    }
}
