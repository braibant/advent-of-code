use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn run(filename: String) {
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
