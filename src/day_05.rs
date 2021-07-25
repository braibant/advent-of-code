use super::intcode;

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(&filename);

    // part 1
    let mut vm = intcode::T::new(&program);
    vm.push(1);
    intcode::execute(&mut vm);
    for o in vm.output.iter() {
        println!("OUTPUT: {}", o)
    }

    // part 2
    let mut vm = intcode::T::new(&program);
    vm.push(5);
    intcode::execute(&mut vm);
    for o in vm.output.iter() {
        println!("OUTPUT: {}", o)
    }
}
