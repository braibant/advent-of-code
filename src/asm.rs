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
