#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustRelativeBase = 9,
    Halt = 99,
}

fn parameters(opcode: Opcode) -> usize {
    match opcode {
        Opcode::Add | Opcode::Mul => 3,
        Opcode::Input | Opcode::Output => 1,
        Opcode::JumpIfTrue | Opcode::JumpIfFalse => 2,
        Opcode::LessThan | Opcode::Equals => 3,
        Opcode::AdjustRelativeBase => 1,
        Opcode::Halt => 0,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    modes: Vec<ParameterMode>,
}

fn decode(instruction: i64) -> Instruction {
    let opcode = match instruction % 100 {
        1 => Opcode::Add,
        2 => Opcode::Mul,
        3 => Opcode::Input,
        4 => Opcode::Output,
        5 => Opcode::JumpIfTrue,
        6 => Opcode::JumpIfFalse,
        7 => Opcode::LessThan,
        8 => Opcode::Equals,
        9 => Opcode::AdjustRelativeBase,
        99 => Opcode::Halt,
        code => panic!("Invalid opcode {}", code),
    };
    let mut modes = vec![];
    let mut acc = instruction / 100;
    for i in 0..parameters(opcode) {
        let mode = match acc % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            n => panic!("Illegal parameter mode {}", n),
        };
        modes.push(mode);
        acc = acc / 10;
    }
    Instruction { opcode, modes }
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
    relative_base: i64,
}

pub type Program = Vec<i64>;

impl T {
    pub fn new(program: &Vec<i64>) -> T {
        T {
            program: program.clone(),
            input: vec![],
            output: vec![],
            status: Status::Continue(0),
            relative_base: 0,
        }
    }

    fn get(&mut self, address: usize) -> i64 {
        if self.program.len() <= address {
            self.program.resize(address + 1, 0);
        };
        self.program[address]
    }

    fn set(&mut self, address: usize, value: i64) {
        if self.program.len() <= address {
            self.program.resize(address + 1, 0)
        };
        self.program[address] = value
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

    pub fn flush(&mut self) -> Vec<i64> {
        let mut acc = vec![];
        while let Some(out) = self.pop() {
            acc.push(out)
        }
        acc
    }

    // Compute the value of the parameter `i` of the `instruction` living at position `pc`. Parameters are 1-indexed.
    fn value(&mut self, instruction: &Instruction, pc: usize, i: usize) -> i64 {
        match &instruction.modes[i - 1] {
            ParameterMode::Immediate => self.get(pc + i),
            ParameterMode::Position => {
                let a = self.get(pc + i) as usize;
                self.get(a)
            }
            ParameterMode::Relative => {
                let a = self.get(pc + i);
                self.get((a + self.relative_base) as usize)
            }
        }
    }

    fn address(&mut self, instruction: &Instruction, pc: usize, i: usize) -> usize {
        match &instruction.modes[i - 1] {
            ParameterMode::Immediate => panic!("Invalid address mode"),
            ParameterMode::Position => self.get(pc + i) as usize,
            ParameterMode::Relative => (self.get(pc + i) + self.relative_base) as usize,
        }
    }

    pub fn step(&mut self, pc: usize) {
        let instruction = decode(self.get(pc));
        let next: Option<Status> = {
            match instruction.opcode {
                Opcode::Halt => Some(Status::Halt),
                Opcode::Add => {
                    let a = self.value(&instruction, pc, 1);
                    let b = self.value(&instruction, pc, 2);
                    let addr = self.address(&instruction, pc, 3);

                    self.set(addr, a + b);
                    None
                }
                Opcode::Mul => {
                    let a = self.value(&instruction, pc, 1);
                    let b = self.value(&instruction, pc, 2);
                    let addr = self.address(&instruction, pc, 3);

                    self.set(addr, a * b);
                    None
                }

                Opcode::Input => {
                    // TODO
                    let addr = self.address(&instruction, pc, 1);
                    if self.input.len() > 0 {
                        let arg = self.input.remove(0);
                        self.set(addr, arg);
                        None
                    } else {
                        Some(Status::Blocked(pc))
                    }
                }
                Opcode::Output => {
                    let v = self.value(&instruction, pc, 1);
                    self.output.push(v);
                    None
                }
                Opcode::JumpIfTrue => {
                    let a = self.value(&instruction, pc, 1);
                    let b = self.value(&instruction, pc, 2);
                    if a != 0 {
                        Some(Status::Continue(b as usize))
                    } else {
                        None
                    }
                }
                Opcode::JumpIfFalse => {
                    let a = self.value(&instruction, pc, 1);
                    let b = self.value(&instruction, pc, 2);
                    if a == 0 {
                        Some(Status::Continue(b as usize))
                    } else {
                        None
                    }
                }
                Opcode::LessThan => {
                    let a = self.value(&instruction, pc, 1);
                    let b = self.value(&instruction, pc, 2);
                    let addr = self.address(&instruction, pc, 3);
                    if a < b {
                        self.set(addr, 1)
                    } else {
                        self.set(addr, 0)
                    };
                    None
                }
                Opcode::Equals => {
                    let a = self.value(&instruction, pc, 1);
                    let b = self.value(&instruction, pc, 2);
                    let addr = self.address(&instruction, pc, 3);
                    if a == b {
                        self.set(addr, 1)
                    } else {
                        self.set(addr, 0)
                    };
                    None
                }
                Opcode::AdjustRelativeBase => {
                    let adjustement = self.value(&instruction, pc, 1);
                    self.relative_base += adjustement;
                    None
                }
            }
        };

        let status = match next {
            None => Status::Continue(pc + 1 + parameters(instruction.opcode)),
            Some(status) => status,
        };

        self.status = status;
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
            Status::Continue(pc) => vm.step(pc),
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
            opcode: Opcode::Mul,
            modes: vec![
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
            ],
        };
        assert_eq!(decode(1002), i);
    }

    #[test]
    fn test_position_mode_1() {
        // outputs 1 if input is 8; outputs 0 otherwise
        let p = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut vm = T::new(&p);
        vm.push(8);
        assert_eq!(vm.pop(), Some(1));

        let mut vm = T::new(&p);
        vm.push(7);
        assert_eq!(vm.pop(), Some(0))
    }

    #[test]
    fn test_position_mode_2() {
        // Outputs 999 if input value is below 8, output 1000 if the value is equal to 8, and outputs 1001 if the value is greater than 8
        let p = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut vm = T::new(&p);
        vm.push(7);
        assert_eq!(vm.pop(), Some(999));

        let mut vm = T::new(&p);
        vm.push(8);
        assert_eq!(vm.pop(), Some(1000));

        let mut vm = T::new(&p);
        vm.push(100);
        assert_eq!(vm.pop(), Some(1001));

        let mut vm = T::new(&p);
        vm.push(-100);
        assert_eq!(vm.pop(), Some(999));
    }

    #[test]
    fn test_quine() {
        let p = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut vm = T::new(&p);
        execute(&mut vm);
        let output = vm.flush();
        assert_eq!(output, p)
    }

    #[test]
    fn test_large_output_1() {
        let p = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut vm = T::new(&p);
        let out = vm.pop().unwrap();
        assert_eq!(out, 1219070632396864);
    }

    #[test]
    fn test_large_output_2() {
        let p = vec![104, 1125899906842624, 99];
        let mut vm = T::new(&p);
        assert_eq!(vm.pop(), Some(1125899906842624));
    }
}
