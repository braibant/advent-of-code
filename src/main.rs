use std::env;

mod day_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Please specify a problem number: 1-25");
        std::process::exit(1);
    } else {
        match args[1].as_str() {
            "1" => day_01::run(args[2].clone()),
            s => {
                println!("Unknown command: {}", s);
                std::process::exit(1)
            }
        }
    }
}
