use std::env;
use std::process;

enum Problem {
    Day1(String),
    Day2(String),
    Day3(String),
    Day4(String),
}

mod day1;
mod day2;
mod day3;
mod day4;

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
        Problem::Day1(filename) => day1::run(filename),
        Problem::Day2(filename) => day2::run(filename),
        Problem::Day3(filename) => day3::run(filename),
        Problem::Day4(filename) => day4::run(filename),
    }
}
