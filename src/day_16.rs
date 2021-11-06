use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Add, Index};

type REG = u64;

#[derive(Copy, Clone)]
enum Instr {
    Add,
    Mul,
    And,
    Or,
    Set,
    Gt,
    Eq,
}

#[derive(Copy, Clone)]
enum Mode {
    Immediate,
    Register,
}

#[derive(Copy, Clone)]
struct Op {
    mnemonic: &'static str,
    instruction: Instr,
    a: Mode,
    b: Mode,
}

const ADDR: Op = Op {
    mnemonic: "ADDR",
    instruction: Instr::Add,
    a: Mode::Register,
    b: Mode::Register,
};

const ADDI: Op = Op {
    mnemonic: "ADDI",
    instruction: Instr::Add,
    a: Mode::Register,
    b: Mode::Immediate,
};

const MULR: Op = Op {
    mnemonic: "MULR",
    instruction: Instr::Mul,
    a: Mode::Register,
    b: Mode::Register,
};
const MULI: Op = Op {
    mnemonic: "MULI",
    instruction: Instr::Mul,
    a: Mode::Register,
    b: Mode::Immediate,
};

const ANDR: Op = Op {
    mnemonic: "ANDR",
    instruction: Instr::And,
    a: Mode::Register,
    b: Mode::Register,
};

const ANDI: Op = Op {
    mnemonic: "ANDI",
    instruction: Instr::And,
    a: Mode::Register,
    b: Mode::Immediate,
};

const ORR: Op = Op {
    mnemonic: "ORR",
    instruction: Instr::Or,
    a: Mode::Register,
    b: Mode::Register,
};
const ORI: Op = Op {
    mnemonic: "ORI",
    instruction: Instr::Or,
    a: Mode::Register,
    b: Mode::Immediate,
};

const SETR: Op = Op {
    mnemonic: "SETR",
    instruction: Instr::Set,
    a: Mode::Register,
    b: Mode::Register,
};
const SETI: Op = Op {
    mnemonic: "SETI",
    instruction: Instr::Set,
    a: Mode::Immediate,
    b: Mode::Immediate,
};

const GTIR: Op = Op {
    mnemonic: "GTIR",
    instruction: Instr::Gt,
    a: Mode::Immediate,
    b: Mode::Register,
};
const GTRI: Op = Op {
    mnemonic: "GTRI",
    instruction: Instr::Gt,
    a: Mode::Register,
    b: Mode::Immediate,
};
const GTRR: Op = Op {
    mnemonic: "GTRR",
    instruction: Instr::Gt,
    a: Mode::Register,
    b: Mode::Register,
};

const EQIR: Op = Op {
    mnemonic: "EQIR",
    instruction: Instr::Eq,
    a: Mode::Immediate,
    b: Mode::Register,
};
const EQRI: Op = Op {
    mnemonic: "EQRI",
    instruction: Instr::Eq,
    a: Mode::Register,
    b: Mode::Immediate,
};
const EQRR: Op = Op {
    mnemonic: "EQRR",
    instruction: Instr::Eq,
    a: Mode::Register,
    b: Mode::Register,
};

const ALL: [Op; 16] = [
    ADDR, ADDI, MULR, MULI, ANDR, ANDI, ORR, ORI, SETR, SETI, GTIR, GTRI, GTRR, EQIR, EQRI, EQRR,
];

#[derive(PartialEq, Debug)]
struct Sample {
    before: [REG; 4],
    after: [REG; 4],
    opcode: REG,
    a: REG,
    b: REG,
    c: REG,
}

fn value(value: REG, mode: Mode, registers: &[REG; 4]) -> Option<REG> {
    match mode {
        Mode::Register => {
            if value < 4 {
                Some(registers[value as usize])
            } else {
                None
            }
        }
        Mode::Immediate => Some(value),
    }
}

fn eval(op: &Op, a: REG, b: REG, c: REG, registers: &[REG; 4]) -> Option<[REG; 4]> {
    let mut registers = registers.clone();
    let a = value(a, op.a, &registers)?;
    let b = value(b, op.b, &registers);
    let val = match op.instruction {
        Instr::Add => {
            let b = b?;
            Some(a + b)
        }
        Instr::Mul => {
            let b = b?;
            Some(a * b)
        }
        Instr::And => {
            let b = b?;
            Some(a & b)
        }
        Instr::Or => {
            let b = b?;
            Some(a | b)
        }
        Instr::Eq => {
            let b = b?;
            if (a == b) {
                Some(1)
            } else {
                Some(0)
            }
        }
        Instr::Gt => {
            let b = b?;
            if (a > b) {
                Some(1)
            } else {
                Some(0)
            }
        }
        Instr::Set => Some(a),
    }?;
    registers[c as usize] = val;
    Some(registers)
}

fn compatible_codes(sample: &Sample) -> HashSet<usize> {
    (0..16)
        .filter_map(|i| {
            if eval(&ALL[i], sample.a, sample.b, sample.c, &sample.before) == Some(sample.after) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

use scan_fmt::scan_fmt;
fn parse_registers(s: &str) -> [REG; 4] {
    let (a, b, c, d) = scan_fmt!(s, "[{}, {}, {}, {}]", REG, REG, REG, REG).unwrap();
    [a, b, c, d]
}

fn parse_instruction(s: &str) -> (REG, REG, REG, REG) {
    scan_fmt!(s, "{} {} {} {}", REG, REG, REG, REG).unwrap()
}

struct DInstr {
    op: Op,
    a: REG,
    b: REG,
    c: REG,
}

fn decode_instruction(s: &str, mapping: &HashMap<usize, usize>) -> DInstr {
    let (op, a, b, c) = parse_instruction(s);
    let &op = mapping.get(&(op as usize)).unwrap();
    let op = ALL[op as usize];
    DInstr { op, a, b, c }
}

fn parse_sample(sample: &str) -> Sample {
    let lines: Vec<_> = sample.split('\n').collect();
    let before = parse_registers(lines[0].strip_prefix("Before: ").unwrap());
    let (opcode, a, b, c) = parse_instruction(lines[1]);
    let after = parse_registers(lines[2].strip_prefix("After:  ").unwrap());
    Sample {
        before,
        after,
        opcode,
        a,
        b,
        c,
    }
}

fn part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .filter(|s| compatible_codes(s).len() >= 3)
        .count()
}

fn mapping(samples: &[Sample]) -> HashMap<usize, usize> {
    // It's easy to mix up numeric IDs here:
    // - compatible_codes returns a list of plain IDs.
    // - candidates contain the list of plain IDs for each cypher ID.
    let mut candidates: Vec<_> = (0..16)
        .map(|_i| {
            let all: HashSet<_> = (0..16).collect();
            all
        })
        .collect();

    for s in samples {
        let codes = compatible_codes(s);
        candidates[s.opcode as usize].retain(|x| codes.contains(x));
    }

    // for i in 0..16 {
    //     println!("{} {:?}", i, candidates[i]);
    // }

    // // Sort the candidates table by decreasing cardinal.
    let mut cypher_to_plain_candidates: Vec<_> = candidates.into_iter().enumerate().collect();

    let mut used = HashSet::new();
    let mut cypher_to_plain = HashMap::new();
    while !cypher_to_plain_candidates.is_empty() {
        cypher_to_plain_candidates
            .iter_mut()
            .for_each(|(_, c)| c.retain(|x| !used.contains(x)));

        cypher_to_plain_candidates.sort_by_key(|(cypher, candidates)| -(candidates.len() as i32));
        let (cypher, mut candidates) = cypher_to_plain_candidates.pop().unwrap();
        assert!(candidates.len() == 1);
        let plain = candidates.into_iter().next().unwrap();
        cypher_to_plain.insert(cypher, plain);
        used.insert(plain);
    }

    cypher_to_plain
}

fn execute(program: &[DInstr]) -> [REG; 4] {
    let mut regs = [0, 0, 0, 0];
    for i in program.iter() {
        regs = eval(&i.op, i.a, i.b, i.c, &regs).unwrap()
    }
    regs
}

pub fn run(s: &str) {
    let contents = std::fs::read_to_string(s).unwrap();
    let parts: Vec<_> = contents.split("\n\n\n\n").collect();

    let samples: Vec<_> = parts[0].split("\n\n").map(|s| parse_sample(s)).collect();
    println!("{}", part1(&samples));
    let cypher_to_plain = mapping(&samples);

    let program: Vec<_> = parts[1]
        .lines()
        .map(|l| decode_instruction(l, &cypher_to_plain))
        .collect();

    let regs = execute(&program);
    println!("{:?}", regs);
}

#[cfg(test)]
mod tests {
    use super::*;

    const S1: &str = "Before: [2, 2, 1, 2]
4 0 3 1
After:  [2, 1, 1, 2]";
    #[test]
    fn test_parse_sample() {
        assert_eq!(
            parse_sample(S1),
            Sample {
                before: [2, 2, 1, 2],
                after: [2, 1, 1, 2],
                opcode: 4,
                a: 0,
                b: 3,
                c: 1
            }
        )
    }

    #[test]
    fn test_eval() {}
}
