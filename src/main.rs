use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

mod day_10;
mod day_11;
mod day_12;
mod day_13;

mod intcode;
mod vector3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 && !(args.len() == 2 && args[1] == "12") {
        println!("Usage: {} DAY INPUT", args[0]);
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "1" => day_01::run(args[2].clone()),
            "2" => day_02::run(args[2].clone()),
            "3" => day_03::run(args[2].clone()),
            "4" => day_04::run(args[2].clone()),
            "5" => day_05::run(args[2].clone()),
            "6" => day_06::run(args[2].clone()),
            "7" => day_07::run(args[2].clone()),
            "8" => day_08::run(args[2].clone()),
            "9" => day_09::run(args[2].clone()),
            "10" => day_10::run(args[2].clone()),
            "11" => day_11::run(args[2].clone()),
            "12" => day_12::run(),
            "13" => day_13::run(args[2].clone()),
            s => {
                println!("Unknown command: {}", s);
                std::process::exit(1)
            }
        }
    }
}
