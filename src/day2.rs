use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn run(filename: String) {
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
    fn day2_example1() {
        let password = Password::parse("1-3 a: abcde".to_string()).unwrap();
        assert!(Password::check1(&password));
        let password = Password::parse("1-3 b: cdefg".to_string()).unwrap();
        assert!(!(Password::check1(&password)))
    }
}
