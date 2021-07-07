#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    Immediate,
    Position,
}

fn parameters(opcode: i64) -> usize {
    if opcode == 1 || opcode == 2 {
        return 3;
    } else if opcode == 3 || opcode == 4 {
        return 1;
    } else if opcode == 5 || opcode == 6 {
        return 2;
    } else if opcode == 7 || opcode == 8 {
        return 3;
    } else if opcode == 99 {
        return 0;
    } else {
        panic!("Illegal opcode {}", opcode)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    opcode: i64,
    modes: Vec<ParameterMode>,
}

fn decode(instruction: i64) -> Instruction {
    let opcode = instruction % 100;
    let mut modes = vec![];
    let mut acc = instruction / 100;
    for i in 0..parameters(opcode) {
        let mode = match acc % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            n => panic!("Illegal parameter mode {}", n),
        };
        modes.push(mode);
        acc = acc / 10;
    }
    Instruction { opcode, modes }
}

// Compute the value of the parameter `i` of the `instruction` living at position `pc`.
fn value(program: &Vec<i64>, instruction: &Instruction, pc: usize, i: usize) -> i64 {
    let i = i - 1;
    match &instruction.modes[i] {
        ParameterMode::Immediate => program[pc + 1 + i],
        ParameterMode::Position => {
            let a = program[pc + 1 + i] as usize;
            program[a]
        }
    }
}

struct T {
    program: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
}

enum Next {
    Halt,
    IncrInstructionPointer,
    JumpTo(usize),
}

pub fn step(
    program: &mut Vec<i64>,
    pc: usize,
    input: &mut Vec<i64>,
    output: &mut Vec<i64>,
) -> Option<usize> {
    let instruction = decode(program[pc]);
    let next = {
        if instruction.opcode == 99 {
            Next::Halt
        } else if instruction.opcode == 1 || instruction.opcode == 2 {
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            let dest = program[pc + 3] as usize;

            if instruction.opcode == 1 {
                program[dest] = a + b;
            } else {
                program[dest] = a * b;
            };
            Next::IncrInstructionPointer
        } else if instruction.opcode == 3 {
            // input
            let dest = program[pc + 1] as usize;
            program[dest] = input.pop().unwrap();
            Next::IncrInstructionPointer
        } else if instruction.opcode == 4 {
            // output
            let v = value(program, &instruction, pc, 1);
            output.push(v);
            Next::IncrInstructionPointer
        } else if instruction.opcode == 5 {
            // jump if true
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            if a != 0 {
                Next::JumpTo(b as usize)
            } else {
                Next::IncrInstructionPointer
            }
        } else if instruction.opcode == 6 {
            // jump if false
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            if a == 0 {
                Next::JumpTo(b as usize)
            } else {
                Next::IncrInstructionPointer
            }
        } else if instruction.opcode == 7 {
            // less than
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            let dest = program[pc + 3] as usize;

            if a < b {
                program[dest] = 1
            } else {
                program[dest] = 0
            };
            Next::IncrInstructionPointer
        } else if instruction.opcode == 8 {
            // equals
            // less than
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            let dest = program[pc + 3] as usize;

            if a == b {
                program[dest] = 1
            } else {
                program[dest] = 0
            };
            Next::IncrInstructionPointer
        } else {
            panic!("Something went wrong {:?}", instruction)
        }
    };

    match next {
        Next::Halt => None,
        Next::IncrInstructionPointer => Some(pc + 1 + parameters(instruction.opcode)),
        Next::JumpTo(addr) => Some(addr),
    }
}

pub fn execute(program: &mut Vec<i64>, input: &mut Vec<i64>) -> Vec<i64> {
    let mut pc = 0;
    let mut output = vec![];
    while let Some(next) = step(program, pc, input, &mut output) {
        pc = next
    }
    output
}

pub fn read_intcode_program(filename: &str) -> Vec<i64> {
    let contents = std::fs::read_to_string(filename).unwrap();
    let program: Vec<_> = contents
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    program
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode() {
        let i = Instruction {
            opcode: 2,
            modes: vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
            ],
        };
        assert_eq!(decode(1002), i);
    }
}
