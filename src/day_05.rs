use super::intcode::*;

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let program: Vec<_> = contents
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // part 1
    let output = execute(&mut program.clone(), &mut vec![1]);
    for o in output.iter() {
        println!("OUTPUT: {}", o)
    }

    // part 2
    let output = execute(&mut program.clone(), &mut vec![5]);
    for o in output.iter() {
        println!("OUTPUT: {}", o)
    }
}
