pub type RegType = u64;

#[derive(Copy, Clone, Debug)]
pub enum Instr {
    Add,
    Mul,
    And,
    Or,
    Set,
    Gt,
    Eq,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Immediate,
    Register,
    Ignore,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
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
    mnemonic: "ANDR",
    instruction: Instr::And,
    a: Mode::Register,
    b: Mode::Register,
};

pub const ANDI: Op = Op {
    mnemonic: "ANDI",
    instruction: Instr::And,
    a: Mode::Register,
    b: Mode::Immediate,
};

pub const ORR: Op = Op {
    mnemonic: "ORR",
    instruction: Instr::Or,
    a: Mode::Register,
    b: Mode::Register,
};
pub const ORI: Op = Op {
    mnemonic: "ORI",
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

pub fn instr_to_string(instr: (&'static Op, u64, u64, u64)) -> String {
    fn arg(i: u64, m: crate::asm::Mode) -> String {
        match m {
            crate::asm::Mode::Immediate => format!("{:2}", i),
            crate::asm::Mode::Register => format!("r{}", i),
            crate::asm::Mode::Ignore => format!("  "),
        }
    }
    let mut s = String::new();
    let op = instr.0;
    s.push_str(op.mnemonic);
    s.push_str(" ");
    s.push_str(&arg(instr.1, op.a));
    s.push_str(" ");
    s.push_str(&arg(instr.2, op.b));
    s.push_str(" ");
    s.push_str(&format!("r{}", instr.3));
    s
}
