use super::intcode::*;

pub fn run(filename: String) {
    let program = read_intcode_program(&filename);

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
