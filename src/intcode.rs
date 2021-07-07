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

pub enum Status {
    // Intcode interpreter is halted
    Halt,
    // Intcode interpreter is waiting for input, and should resume from instruction pointer once input is available.
    Blocked(usize),
    // Continue execution from instruction pointer
    Continue(usize),
}

pub struct T {
    pub program: Vec<i64>,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pub status: Status,
}

pub fn step(
    program: &mut Vec<i64>,
    pc: usize,
    input: &mut Vec<i64>,
    output: &mut Vec<i64>,
) -> Status {
    let instruction = decode(program[pc]);
    let next: Option<Status> = {
        if instruction.opcode == 99 {
            Some(Status::Halt)
        } else if instruction.opcode == 1 || instruction.opcode == 2 {
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            let dest = program[pc + 3] as usize;

            if instruction.opcode == 1 {
                program[dest] = a + b;
            } else {
                program[dest] = a * b;
            };
            None
        } else if instruction.opcode == 3 {
            // input
            let dest = program[pc + 1] as usize;
            if input.len() > 0 {
                let arg = input.remove(0);
                program[dest] = arg;
                None
            } else {
                Some(Status::Blocked(pc))
            }
        } else if instruction.opcode == 4 {
            // output
            let v = value(program, &instruction, pc, 1);
            output.push(v);
            None
        } else if instruction.opcode == 5 {
            // jump if true
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            if a != 0 {
                Some(Status::Continue(b as usize))
            } else {
                None
            }
        } else if instruction.opcode == 6 {
            // jump if false
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            if a == 0 {
                Some(Status::Continue(b as usize))
            } else {
                None
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
            None
        } else if instruction.opcode == 8 {
            // equals
            let a = value(program, &instruction, pc, 1);
            let b = value(program, &instruction, pc, 2);
            let dest = program[pc + 3] as usize;

            if a == b {
                program[dest] = 1
            } else {
                program[dest] = 0
            };
            None
        } else {
            panic!("Something went wrong {:?}", instruction)
        }
    };

    match next {
        None => Status::Continue(pc + 1 + parameters(instruction.opcode)),
        Some(status) => status,
    }
}

// Execute the program until it is blocked or halted.
pub fn execute(vm: &mut T) {
    loop {
        match vm.status {
            Status::Halt => break,
            Status::Blocked(pc) => {
                if vm.input.len() > 0 {
                    vm.status = Status::Continue(pc)
                } else {
                    break;
                }
            }
            Status::Continue(pc) => {
                vm.status = step(&mut vm.program, pc, &mut vm.input, &mut vm.output)
            }
        }
    }
}

impl T {
    pub fn new(program: &Vec<i64>) -> T {
        T {
            program: program.clone(),
            input: vec![],
            output: vec![],
            status: Status::Continue(0),
        }
    }

    pub fn push(&mut self, i: i64) {
        self.input.push(i)
    }

    pub fn pop(&mut self) -> Option<i64> {
        execute(self);
        if self.output.len() > 0 {
            Some(self.output.remove(0))
        } else {
            None
        }
    }

    pub fn is_halted(&mut self) -> bool {
        execute(self);
        match self.status {
            Status::Halt => true,
            _ => false,
        }
    }
}

pub fn from_string(program: &str) -> Vec<i64> {
    let program: Vec<_> = program
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    program
}

pub fn read_intcode_program(filename: &str) -> Vec<i64> {
    let contents = std::fs::read_to_string(filename).unwrap();
    from_string(&contents)
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
