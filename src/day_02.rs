fn parameters(opcode: usize) -> usize {
    return 3;
}

fn step(program: &mut Vec<usize>, pc: usize) -> Option<usize> {
    let opcode = program[pc];
    if opcode == 99 {
        return None;
    } else if opcode == 1 || opcode == 2 {
        let a = program[pc + 1];
        let b = program[pc + 2];
        let dest = program[pc + 3];
        if opcode == 1 {
            program[dest] = program[a] + program[b]
        } else {
            assert_eq!(opcode, 2);
            program[dest] = program[a] * program[b]
        };
        Some(pc + 1 + parameters(opcode))
    } else {
        panic!("Something went wrong")
    }
}

fn execute(program: &mut Vec<usize>) {
    let mut pc = 0;
    while let Some(next) = step(program, pc) {
        pc = next
    }
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let program: Vec<_> = contents
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // part 1
    let mut p1 = program.clone();
    p1[1] = 12;
    p1[2] = 2;
    execute(&mut p1);
    println!("{}", p1[0]);

    // part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let mut p2 = program.clone();
            p2[1] = noun;
            p2[2] = verb;

            execute(&mut p2);
            if p2[0] == 19690720 {
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
        let mut p = vec![1, 0, 0, 0, 99];
        execute(&mut p);
        assert_eq!(p, vec![2, 0, 0, 0, 99])
    }

    #[test]
    fn test_2() {
        let mut p = vec![2, 3, 0, 3, 99];
        execute(&mut p);
        assert_eq!(p, vec![2, 3, 0, 6, 99])
    }

    #[test]
    fn test_3() {
        let mut p = vec![2, 4, 4, 5, 99, 0];
        execute(&mut p);
        assert_eq!(p, vec![2, 4, 4, 5, 99, 9801])
    }
}
