use std::env;

mod util;

mod day_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} DAY INPUT", args[0]);
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "01" => day_01::run(&args[2]),
            s => {
                println!("Unknown command: {}", s);
                std::process::exit(1)
            }
        }
    }
}
