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
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

mod intcode;
mod vector2;
mod vector3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 && !(args.len() == 2 && args[1] == "12") {
        println!("Usage: {} DAY INPUT", args[0]);
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "1" => day_01::run(&args[2]),
            "2" => day_02::run(&args[2]),
            "3" => day_03::run(&args[2]),
            "4" => day_04::run(&args[2]),
            "5" => day_05::run(&args[2]),
            "6" => day_06::run(&args[2]),
            "7" => day_07::run(&args[2]),
            "8" => day_08::run(&args[2]),
            "9" => day_09::run(&args[2]),
            "10" => day_10::run(&args[2]),
            "11" => day_11::run(&args[2]),
            "12" => day_12::run(),
            "13" => day_13::run(&args[2]),
            "14" => day_14::run(&args[2]),
            "15" => day_15::run(&args[2]),
            "16" => day_16::run(&args[2]),
            "17" => day_17::run(&args[2]),
            "18" => day_18::run(&args[2]),
            "19" => day_19::run(&args[2]),
            "20" => day_20::run(&args[2]),
            s => {
                println!("Unknown command: {}", s);
                std::process::exit(1)
            }
        }
    }
}
