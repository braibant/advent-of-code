use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

mod day_05;
mod day_06;
mod intcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Please specify a problem number: 1-25");
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "1" => day_01::run(args[2].clone()),
            "2" => day_02::run(args[2].clone()),
            "3" => day_03::run(args[2].clone()),
            "4" => day_04::run(args[2].clone()),
            "5" => day_05::run(args[2].clone()),
            "6" => day_06::run(args[2].clone()),
            s => {
                println!("Unknown command: {}", s);
                std::process::exit(1)
            }
        }
    }
}
