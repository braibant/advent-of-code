use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[2].clone();
    match args[1].as_str() {
        "1" => day1::run(filename),
        "2" => day2::run(filename),
        "3" => day3::run(filename),
        "4" => day4::run(filename),
        "5" => day5::run(filename),
        "6" => day6::run(filename),
        "7" => day7::run(filename),
        "8" => day8::run(filename),
        "9" => day9::run(filename),
        _ => panic!("Not yet implemented"),
    }
}
