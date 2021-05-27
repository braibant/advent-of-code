use std::env;

#[macro_use]
extern crate scan_fmt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
//
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
//
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        "10" => day10::run(filename),
        "11" => day11::run(filename),
        "12" => day12::run(filename),
        "13" => day13::run(filename),
        "14" => day14::run(filename),
        "15" => day15::run(),
        "16" => day16::run(filename),
        "17" => day17::run(filename),
        "18" => day18::run(filename),
        "19" => day19::run(filename),
        "20" => day20::run(filename),
        "21" => day21::run(filename),
        "22" => day22::run(filename),
        "23" => day23::run(filename),
        "24" => day24::run(filename),
        "25" => day25::run(filename),
        _ => panic!("Not yet implemented"),
    }
}
