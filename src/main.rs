use std::env;

mod util;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} DAY INPUT", args[0]);
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "01" => day_01::run(&args[2]),
            "02" => day_02::run(&args[2]),
            "03" => day_03::run(&args[2]),
            "04" => day_04::run(&args[2]),
            s => {
                println!("Unknown command: {}", s);
                std::process::exit(1)
            }
        }
    }
}
