#![allow(unused_imports)]

use std::env;

#[macro_use]
extern crate scan_fmt;

#[macro_use]
extern crate lazy_static;

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
    if args.len() < 1 {
        println!("Please specify a problem number: 1-25");
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "1" => {
                let filename = args[2].clone();
                day1::run(filename)
            }
            "2" => {
                let filename = args[2].clone();
                day2::run(filename)
            }
            "3" => {
                let filename = args[2].clone();
                day3::run(filename)
            }
            "4" => {
                let filename = args[2].clone();
                day4::run(filename)
            }
            "5" => {
                let filename = args[2].clone();
                day5::run(filename)
            }
            "6" => {
                let filename = args[2].clone();
                day6::run(filename)
            }
            "7" => {
                let filename = args[2].clone();
                day7::run(filename)
            }
            "8" => {
                let filename = args[2].clone();
                day8::run(filename)
            }
            "9" => {
                let filename = args[2].clone();
                day9::run(filename)
            }
            "10" => {
                let filename = args[2].clone();
                day10::run(filename)
            }
            "11" => {
                let filename = args[2].clone();
                day11::run(filename)
            }
            "12" => {
                let filename = args[2].clone();
                day12::run(filename)
            }
            "13" => {
                let filename = args[2].clone();
                day13::run(filename)
            }
            "14" => {
                let filename = args[2].clone();
                day14::run(filename)
            }
            "15" => day15::run(),
            "16" => {
                let filename = args[2].clone();
                day16::run(filename)
            }
            "17" => {
                let filename = args[2].clone();
                day17::run(filename)
            }
            "18" => {
                let filename = args[2].clone();
                day18::run(filename)
            }
            "19" => {
                let filename = args[2].clone();
                day19::run(filename)
            }
            "20" => {
                let filename = args[2].clone();
                day20::run(filename)
            }
            "21" => {
                let filename = args[2].clone();
                day21::run(filename)
            }
            "22" => {
                let filename = args[2].clone();
                day22::run(filename)
            }
            "23" => day23::run(),
            "24" => {
                let filename = args[2].clone();
                day24::run(filename)
            }
            "25" => {
                let filename = args[2].clone();
                day25::run(filename)
            }
            s => panic!("Invalid problem: {}", s),
        }
    }
}
