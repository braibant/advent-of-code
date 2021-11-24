use crate::asm::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
struct Sample {
    before: [RegType; 4],
    after: [RegType; 4],
    opcode: RegType,
    a: RegType,
    b: RegType,
    c: RegType,
}

fn value(value: RegType, mode: Mode, registers: &[RegType; 4]) -> Option<RegType> {
    match mode {
        Mode::Register => {
            if value < 4 {
                Some(registers[value as usize])
            } else {
                None
            }
        }
        Mode::Ignore => None,
        Mode::Immediate => Some(value),
    }
}

fn eval(
    op: &Op,
    a: RegType,
    b: RegType,
    c: RegType,
    registers: &[RegType; 4],
) -> Option<[RegType; 4]> {
    let mut registers = *registers;
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
            if a == b {
                Some(1)
            } else {
                Some(0)
            }
        }
        Instr::Gt => {
            let b = b?;
            if a > b {
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
        .filter(|i| {
            eval(&ALL[*i], sample.a, sample.b, sample.c, &sample.before) == Some(sample.after)
        })
        .collect()
}

use scan_fmt::scan_fmt;
fn parse_registers(s: &str) -> [RegType; 4] {
    let tuple = scan_fmt!(s, "[{}, {}, {}, {}]", RegType, RegType, RegType, RegType).unwrap();
    [tuple.0, tuple.1, tuple.2, tuple.3]
}

fn parse_instruction(s: &str) -> (RegType, RegType, RegType, RegType) {
    scan_fmt!(s, "{} {} {} {}", RegType, RegType, RegType, RegType).unwrap()
}

struct DInstr {
    op: Op,
    a: RegType,
    b: RegType,
    c: RegType,
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

        cypher_to_plain_candidates.sort_by_key(|(_cypher, candidates)| -(candidates.len() as i32));
        let (cypher, candidates) = cypher_to_plain_candidates.pop().unwrap();
        assert!(candidates.len() == 1);
        let plain = candidates.into_iter().next().unwrap();
        cypher_to_plain.insert(cypher, plain);
        used.insert(plain);
    }

    cypher_to_plain
}

fn execute(program: &[DInstr]) -> [RegType; 4] {
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
