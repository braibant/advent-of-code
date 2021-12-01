use std::collections::HashSet;

pub type RegType = u64;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instr {
    Add,
    Mul,
    And,
    Or,
    Set,
    Gt,
    Eq,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Immediate,

    Register,
    Ignore,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Op {
    pub mnemonic: &'static str,
    pub instruction: Instr,
    pub a: Mode,
    pub b: Mode,
}

pub const ADDR: Op = Op {
    mnemonic: "ADDR",
    instruction: Instr::Add,
    a: Mode::Register,
    b: Mode::Register,
};

pub const ADDI: Op = Op {
    mnemonic: "ADDI",
    instruction: Instr::Add,
    a: Mode::Register,
    b: Mode::Immediate,
};

pub const MULR: Op = Op {
    mnemonic: "MULR",
    instruction: Instr::Mul,
    a: Mode::Register,
    b: Mode::Register,
};
pub const MULI: Op = Op {
    mnemonic: "MULI",
    instruction: Instr::Mul,
    a: Mode::Register,
    b: Mode::Immediate,
};

pub const ANDR: Op = Op {
    mnemonic: "BANR",
    instruction: Instr::And,
    a: Mode::Register,
    b: Mode::Register,
};

pub const ANDI: Op = Op {
    mnemonic: "BANI",
    instruction: Instr::And,
    a: Mode::Register,
    b: Mode::Immediate,
};

pub const ORR: Op = Op {
    mnemonic: "BORR",
    instruction: Instr::Or,
    a: Mode::Register,
    b: Mode::Register,
};
pub const ORI: Op = Op {
    mnemonic: "BORI",
    instruction: Instr::Or,
    a: Mode::Register,
    b: Mode::Immediate,
};

pub const SETR: Op = Op {
    mnemonic: "SETR",
    instruction: Instr::Set,
    a: Mode::Register,
    b: Mode::Ignore,
};
pub const SETI: Op = Op {
    mnemonic: "SETI",
    instruction: Instr::Set,
    a: Mode::Immediate,
    b: Mode::Ignore,
};

pub const GTIR: Op = Op {
    mnemonic: "GTIR",
    instruction: Instr::Gt,
    a: Mode::Immediate,
    b: Mode::Register,
};
pub const GTRI: Op = Op {
    mnemonic: "GTRI",
    instruction: Instr::Gt,
    a: Mode::Register,
    b: Mode::Immediate,
};
pub const GTRR: Op = Op {
    mnemonic: "GTRR",
    instruction: Instr::Gt,
    a: Mode::Register,
    b: Mode::Register,
};

pub const EQIR: Op = Op {
    mnemonic: "EQIR",
    instruction: Instr::Eq,
    a: Mode::Immediate,
    b: Mode::Register,
};
pub const EQRI: Op = Op {
    mnemonic: "EQRI",
    instruction: Instr::Eq,
    a: Mode::Register,
    b: Mode::Immediate,
};
pub const EQRR: Op = Op {
    mnemonic: "EQRR",
    instruction: Instr::Eq,
    a: Mode::Register,
    b: Mode::Register,
};

pub const ALL: [Op; 16] = [
    ADDR, ADDI, MULR, MULI, ANDR, ANDI, ORR, ORI, SETR, SETI, GTIR, GTRI, GTRR, EQIR, EQRI, EQRR,
];

pub type Program = Vec<(&'static Op, u64, u64, u64)>;

pub fn parse(s: &str) -> (Option<usize>, Program) {
    let mut ip = None;
    let mut program: Program = Vec::new();
    for line in s.split('\n').filter(|l| !l.is_empty()) {
        if line.starts_with("#ip ") {
            if let Some(l) = line.strip_prefix("#ip ") {
                ip = Some(l.parse().unwrap());
            }
        } else {
            let elements: Vec<_> = line.split(' ').collect();
            let op = crate::asm::ALL
                .iter()
                .find(|op| op.mnemonic.to_lowercase() == elements[0])
                .unwrap();
            let a = elements[1].parse().unwrap();
            let b = elements[2].parse().unwrap();
            let c = elements[3].parse().unwrap();
            program.push((op, a, b, c));
        }
    }
    (ip, program)
}

pub fn instr_to_string(ipr: usize, instr: (&'static Op, u64, u64, u64)) -> String {
    let binary = match instr.0.instruction {
        Instr::And | Instr::Or => true,
        _ => false,
    };
    let arg = |i: u64, m: crate::asm::Mode| -> String {
        match m {
            crate::asm::Mode::Immediate => {
                if binary {
                    format!("0b{:b}", i)
                } else {
                    format!("{:2}", i)
                }
            }
            crate::asm::Mode::Register => format!("r{}", i),
            crate::asm::Mode::Ignore => format!("  "),
        }
    };
    let mut s = String::new();
    let op = instr.0;
    if *op == SETI && instr.3 == (ipr as u64) {
        s.push_str(&format!("JUMP {}", instr.1 + 1))
    } else {
        s.push_str(op.mnemonic);
        s.push_str(" ");
        s.push_str(&arg(instr.1, op.a));
        s.push_str(" ");
        s.push_str(&arg(instr.2, op.b));
        s.push_str(" ");
        s.push_str(&format!("r{}", instr.3));
    }
    s
}

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

#[derive(Clone)]
pub struct T {
    pub registers: [u64; 6],
    ipr: usize,
    ip: usize,
    pre_ip: Option<usize>,
    text: Program,
    pub halted: bool,
    steps: usize,
    counts: Vec<usize>,
    last: Vec<[u64; 6]>,
    preds: Vec<HashSet<usize>>,
}

impl T {
    pub fn new(text: &Program, ipr: usize) -> T {
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

    pub fn step(&mut self) {
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
    pub fn to_table(&self) -> prettytable::Table {
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
                instr_to_string(self.ipr, *instr),
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
