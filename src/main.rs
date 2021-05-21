use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

enum Problem {
    Day1(String),
    Day2(String),
    Day3(String),
    Day4(String),
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

fn day3(filename: String) {
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

struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    // Assume passports have been normalized as a single line with key:value pairs separated by spaces.
    fn parse(s: &str) -> Option<Passport> {
        let s = s.trim();
        let v: Vec<&str> = s.split(&[' '][..]).collect();
        let mut p = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        for kv in v.iter() {
            let kv: Vec<&str> = kv.split(&[':'][..]).collect();
            match kv[0] {
                "byr" => p.byr = Some(kv[1].to_string().parse().unwrap()),
                "iyr" => p.iyr = Some(kv[1].to_string().parse().unwrap()),
                "eyr" => p.eyr = Some(kv[1].to_string().parse().unwrap()),
                "hgt" => p.hgt = Some(kv[1].to_string()),
                "hcl" => p.hcl = Some(kv[1].to_string()),
                "ecl" => p.ecl = Some(kv[1].to_string()),
                "pid" => p.pid = Some(kv[1].to_string()),
                "cid" => p.cid = Some(kv[1].to_string()),
                k => panic!("Unexpected key in passport {}", k),
            }
        }
        return Some(p);
    }

    fn check1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn check2(&self) -> bool {
        // This check is more restrictive that check1.
        // Let's make sure that all fields are set, before doing stricter validation
        if !self.check1() {
            return false;
        };

        let byr = self.byr.unwrap();
        let iyr = self.iyr.unwrap();
        let eyr = self.eyr.unwrap();
        let hgt = self.hgt.as_ref().unwrap();
        let hcl = self.hcl.as_ref().unwrap();
        let ecl = self.ecl.as_ref().unwrap();
        let pid = self.pid.as_ref().unwrap();

        let c1 = 1920 <= byr && byr <= 2002;
        let c2 = 2010 <= iyr && iyr <= 2020;
        let c3 = 2020 <= eyr && eyr <= 2030;

        let c4 = if hgt.ends_with("cm") {
            let hgt: i32 = hgt.trim_end_matches("cm").parse().unwrap();
            150 <= hgt && hgt <= 193
        } else if hgt.ends_with("in") {
            let hgt: i32 = hgt.trim_end_matches("in").parse().unwrap();
            59 <= hgt && hgt <= 76
        } else {
            false
        };

        let c5 = if hcl.starts_with("#") {
            hcl[1..].chars().all(|x| char::is_ascii_hexdigit(&x))
        } else {
            false
        };

        let c6 = ecl == "amb"
            || ecl == "blu"
            || ecl == "brn"
            || ecl == "gry"
            || ecl == "grn"
            || ecl == "hzl"
            || ecl == "oth";

        let c7 = pid.len() == 9 && pid.chars().all(|x| char::is_ascii_digit(&x));
        c1 && c2 && c3 && c4 && c5 && c6 && c7
    }
}

fn day4(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut passports: Vec<Passport> = Vec::new();
    let mut acc = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            passports.push(Passport::parse(&acc).unwrap());
            acc.clear();
        } else {
            acc.push_str(&line);
            acc.push(' ');
        }
    }
    // deal with files without trailing new line
    if acc.len() > 0 {
        passports.push(Passport::parse(&acc).unwrap())
    }

    let mut count1 = 0;
    let mut count2 = 0;
    for p in passports.iter() {
        if p.check1() {
            count1 += 1
        };
        if p.check2() {
            count2 += 1
        };
    }
    println!("{}", count1);
    println!("{}", count2);
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
impl Problem {
    fn new(args: &[String]) -> Result<Problem, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        };
        let day = args[1].clone();
        match day.as_str() {
            "1" => {
                return Ok(Problem::Day1(args[2].clone()));
            }
            "2" => {
                return Ok(Problem::Day2(args[2].clone()));
            }
            "3" => {
                return Ok(Problem::Day3(args[2].clone()));
            }
            "4" => {
                return Ok(Problem::Day4(args[2].clone()));
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
        Problem::Day3(filename) => day3(filename),
        Problem::Day4(filename) => day4(filename),
    }
}
