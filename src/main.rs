use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

enum Problem {
    Day1(String),
}

fn day1(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num: i32 = line.parse().unwrap();
        content.push(num);
    }

    content.sort();

    let mut i = 0;
    let mut j = content.len() - 1;
    while i < j {
        if (content[i] + content[j] < 2020) {
            i += 1
        } else if (content[i] + content[j] > 2020) {
            j -= 1
        } else {
            println!("{}", content[i] * content[j]);
            return;
        }
    }

    panic!("Invariant does not hold")
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
    }
}
