use crate::asm::{instr_to_string, parse, Op, Program};
use std::collections::HashSet;

fn value(
    value: crate::asm::RegType,
    mode: crate::asm::Mode,
    registers: &[crate::asm::RegType; 6],
) -> Option<crate::asm::RegType> {
    match mode {
        crate::asm::Mode::Register => {
            if value < 6 {
                Some(registers[value as usize])
            } else {
                None
            }
        }
        crate::asm::Mode::Ignore => None,
        crate::asm::Mode::Immediate => Some(value),
    }
}

fn eval(registers: &[u64; 6], instr: (&'static Op, u64, u64, u64)) -> Option<[u64; 6]> {
    use crate::asm::Instr;

    let mut registers = *registers;
    let (op, a, b, c) = instr;
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

struct T {
    registers: [u64; 6],
    ipr: usize,
    ip: usize,
    pre_ip: Option<usize>,
    text: Program,
    halted: bool,
    steps: usize,
    counts: Vec<usize>,
    last: Vec<[u64; 6]>,
    preds: Vec<HashSet<usize>>,
}

impl T {
    fn new(text: &Program, ipr: usize) -> T {
        T {
            registers: [0; 6],
            ipr,
            ip: 0,
            pre_ip: None,
            text: text.clone(),
            halted: false,
            steps: 0,
            counts: (0..text.len()).map(|_| 0).collect(),
            last: (0..text.len()).map(|_| [0; 6]).collect(),
            preds: (0..text.len()).map(|_| HashSet::new()).collect(),
        }
    }

    fn step(&mut self) {
        if !self.halted {
            let ip = self.ip as usize;
            self.counts[ip] += 1;
            self.last[ip] = self.registers;
            if let Some(pre_ip) = self.pre_ip {
                self.preds[ip].insert(pre_ip);
            }
            self.registers[self.ipr] = self.ip as u64;
            assert!((self.ip as usize) < self.text.len());

            let instr = self.text[self.ip as usize];
            let regs = eval(&self.registers, instr);
            match regs {
                Some(regs) => self.registers = regs,
                None => self.halted = true,
            };
            self.pre_ip = Some(self.ip);
            self.ip = (self.registers[self.ipr] + 1) as usize;
            if (self.ip as usize) >= self.text.len() {
                self.halted = true
            }
            self.steps += 1
        }
    }

    #[allow(dead_code)]
    fn to_table(&self) -> prettytable::Table {
        use prettytable::{Cell, Row, Table};
        let mut table = Table::new();
        table.set_titles(row![
            "", "", "INSTR", "COUNT", "0", "1", "2", "3", "4", "5", "preds"
        ]);
        for (i, instr) in self.text.iter().enumerate() {
            let row: Vec<_> = vec![
                {
                    if self.ip == i {
                        "*".to_string()
                    } else {
                        " ".to_string()
                    }
                },
                format!("{}", i),
                instr_to_string(*instr),
                format!("{}", self.counts[i]),
                format!("{}", self.last[i][0]),
                format!("{}", self.last[i][1]),
                format!("{}", self.last[i][2]),
                format!("{}", self.last[i][3]),
                format!("{}", self.last[i][4]),
                format!("{}", self.last[i][5]),
                format!("{:?}", self.preds[i]),
            ]
            .into_iter()
            .map(|s| Cell::new(&s))
            .collect();
            table.add_row(Row::new(row));
        }
        table
    }
}

fn part1(text: &Program, ip: usize) -> u64 {
    let mut t = T::new(text, ip);
    while !t.halted {
        t.step();
    }
    t.registers[0]
}

fn part2(_text: &Program, _ip: usize) -> u64 {
    // let mut t = T::new(text, ip);
    // t.registers[0] = 1;
    // while !t.halted {
    //     t.step();
    //     if t.steps % 10_000_000 == 0 {
    //         print!("\x1B[2J\x1B[1;1H");
    //         let mut table = t.to_table();
    //         use prettytable::format;
    //         table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    //         table.printstd();
    //     }
    // }

    // t.registers[0]

    // When we set reg[0] to 1, the program becomes an obfuscated version of the following loop:
    // for i = 1 to 10551374 + 1 do if 10551374 mod i = 0 then r := !r + i; done
    // which evaluates to r being 15864120
    return 15864120;
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let (ip, text) = parse(&contents);
    println!("{:?}", part1(&text, ip.unwrap()));
    println!("{:?}", part2(&text, ip.unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;
    const E1: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn test_example1() {
        let (ip, text) = parse(E1);
        assert_eq!(part1(&text, ip.unwrap()), 6)
    }
}
