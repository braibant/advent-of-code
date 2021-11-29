use crate::asm::{instr_to_string, parse, Op, Program};
use prettytable;
use std::collections::HashSet;

const N: usize = 6;

type Int = u64;

#[derive(Copy, Clone)]
struct Interval {
    low: Int,
    high: Int,
}

impl Interval {
    fn scalar(t: Int) -> Self {
        Interval { low: t, high: t }
    }

    fn new(low: Int, high: Int) -> Self {
        Interval { low, high }
    }

    fn add(&self, other: Self) -> Self {
        Interval {
            low: self.low + other.low,
            high: self.high + other.high,
        }
    }

    fn mul(&self, other: Self) -> Self {
        Interval {
            low: self.low * other.low,
            high: self.high * other.high,
        }
    }

    fn and(&self, other: Self) -> Self {
        Interval {
            low: 0,
            high: std::cmp::min(self.high, other.high).next_power_of_two() - 1,
        }
    }

    fn or(&self, other: Self) -> Self {
        Interval {
            low: 0,
            high: std::cmp::max(self.high, other.high).next_power_of_two() - 1,
        }
    }

    fn is_scalar(&self) -> Option<Int> {
        if self.low == self.high {
            Some(self.low)
        } else {
            None
        }
    }

    fn contains(&self, pt: Int) -> bool {
        self.low <= pt && pt <= self.high
    }

    fn overlap(&self, other: Self) -> bool {
        other.contains(self.low)
            || other.contains(self.high)
            || self.contains(other.low)
            || self.contains(other.high)
    }

    fn eq(&self, other: Self) -> Self {
        let yes = Self::scalar(1);
        let no = Self::scalar(0);
        let maybe = Self::new(0, 1);
        match (self.is_scalar(), other.is_scalar()) {
            (Some(a), Some(b)) => {
                if (a == b) {
                    yes
                } else {
                    no
                }
            }
            (_, _) => {
                if self.overlap(other) {
                    maybe
                } else {
                    no
                }
            }
        }
    }

    fn gt(&self, other: Self) -> Self {
        let yes = Self::scalar(1);
        let no = Self::scalar(0);
        let maybe = Self::new(0, 1);
        match (self.is_scalar(), other.is_scalar()) {
            (Some(a), Some(b)) => {
                if (a > b) {
                    yes
                } else {
                    no
                }
            }
            (_, _) => {
                if self.low > other.high {
                    yes
                } else if self.high <= other.low {
                    no
                } else {
                    maybe
                }
            }
        }
    }
}

// We want to map each program point to an abstract state that contains all the
// concrete states.

type Domain = Interval;

#[derive(Clone, Copy)]
struct Registers([Domain; 6]);

fn value(value: Int, mode: crate::asm::Mode, registers: &Registers) -> Option<Domain> {
    use crate::asm::Mode::*;
    match mode {
        Register => {
            if value < 6 {
                Some(registers.0[value as usize])
            } else {
                None
            }
        }
        Ignore => None,
        Immediate => Some(Interval::scalar(value)),
    }
}

fn instr(registers: &Registers, instr: (&'static Op, Int, Int, Int)) -> Option<Registers> {
    use crate::asm::Instr;
    let (op, a, b, c) = instr;
    let a = value(a, op.a, registers)?;
    let b = value(b, op.b, registers);
    let val = match op.instruction {
        Instr::Add => {
            let b = b?;
            Some(a.add(b))
        }
        Instr::Mul => {
            let b = b?;
            Some(a.mul(b))
        }
        Instr::And => {
            let b = b?;
            Some(a.and(b))
        }
        Instr::Or => {
            let b = b?;
            Some(a.or(b))
        }
        Instr::Eq => {
            let b = b?;
            Some(a.eq(b))
        }
        Instr::Gt => {
            let b = b?;
            Some(a.gt(b))
        }
        Instr::Set => Some(a),
    }?;
    let mut regs = registers.clone();
    let c = if c < 6 { Some(c as usize) } else { None }?;
    regs.0[c] = val;
    Some(regs)
}

struct T {
    state: Vec<Option<Registers>>,
}


impl T {
}
