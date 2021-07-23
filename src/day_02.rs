use super::intcode;

pub fn run(filename: String) {
    let program = intcode::read_intcode_program(&filename);
    // part 1

    let mut vm = intcode::T::new(&program);
    vm.program[1] = 12;
    vm.program[2] = 2;
    intcode::execute(&mut vm);
    println!("{}", vm.program[0]);

    // part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm = intcode::T::new(&program);
            vm.program[1] = noun;
            vm.program[2] = verb;
            intcode::execute(&mut vm);
            if vm.program[0] == 19690720 {
                println!("{}", 100 * noun + verb)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let p = vec![1, 0, 0, 0, 99];
        let mut vm = intcode::T::new(&p);
        intcode::execute(&mut vm);
        assert_eq!(vm.program, vec![2, 0, 0, 0, 99])
    }

    #[test]
    fn test_2() {
        let p = vec![2, 3, 0, 3, 99];
        let mut vm = intcode::T::new(&p);
        intcode::execute(&mut vm);
        assert_eq!(vm.program, vec![2, 3, 0, 6, 99])
    }

    #[test]
    fn test_3() {
        let p = vec![2, 4, 4, 5, 99, 0];
        let mut vm = intcode::T::new(&p);
        intcode::execute(&mut vm);
        assert_eq!(vm.program, vec![2, 4, 4, 5, 99, 9801])
    }
}
