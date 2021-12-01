use crate::intcode;

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(&filename);
    let mut vm = intcode::T::new(&program);
    vm.push(1);
    let output = vm.get_outputs();

    println!("{:?}", output);

    let mut vm = intcode::T::new(&program);
    vm.push(2);
    let output = vm.get_outputs();

    println!("{:?}", output);
}
