use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct u36(u64);

impl fmt::Debug for u36 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&format!("{:b}", self.0))
    }
}

#[derive(Debug)]
enum Instruction {
    // Use three masks to avoid some bit fiddling later on
    Mask(u36, u36, u36),
    Mem(u64, u64),
}

fn parse(s: &str) -> Instruction {
    if let Ok(mask) = scan_fmt!(s, "mask = {}", String) {
        let mut maskx = 0;
        let mut mask1 = 0;
        let mut mask0 = 0;
        let mut i = 1 << 35;
        for c in mask.chars() {
            if c == 'X' {
                maskx |= i
            } else if c == '1' {
                mask1 |= i
            } else if c == '0' {
                mask0 |= i
            } else {
                panic!("Parsing failure {}", s)
            }
            i = i >> 1;
        }
        return Instruction::Mask(u36(mask1), u36(mask0), u36(maskx));
    };
    if let Ok((addr, value)) = scan_fmt!(s, "mem[{d}] = {d}", u64, u64) {
        return Instruction::Mem(addr, value);
    }
    panic!("Parsing failure {}", s)
}

fn part1(prog: &Vec<Instruction>) {
    // The program starts by specifying a bitmask.
    let mut mask1 = 0;
    let mut mask0 = 0;

    let mut mem = HashMap::new();
    for i in prog.iter() {
        match i {
            Instruction::Mask(u36(m1), u36(m0), _) => {
                mask1 = *m1;
                mask0 = *m0;
            }
            Instruction::Mem(addr, value) => {
                let value = value | mask1;
                let value = value & (!mask0);
                mem.insert(addr, value);
            }
        }
    }

    let sum: u64 = mem.into_iter().map(|(_key, value)| value).sum();
    println!("{}", sum);
}

// Recursive function that iterates through the floating part of the address.
fn set(mem: &mut HashMap<u64, u64>, addr: u64, maskx: u64, value: u64) {
    if maskx == 0 {
        mem.insert(addr, value);
    } else {
        let rightmost_one_bit = maskx & (maskx.wrapping_neg());
        // drop the rightmost one bit
        let maskx = maskx & !rightmost_one_bit;
        let addr1 = addr | rightmost_one_bit;
        let addr0 = addr & (!rightmost_one_bit);
        set(mem, addr1, maskx, value);
        set(mem, addr0, maskx, value)
    }
}

fn part2(prog: &Vec<Instruction>) {
    // The program starts by specifying a bitmask.
    let mut mask1 = 0;
    let mut maskx = 0;
    let mut mem = HashMap::new();
    for (_index, instruction) in prog.iter().enumerate() {
        match instruction {
            Instruction::Mask(u36(m1), u36(_m0), u36(mx)) => {
                mask1 = *m1;
                maskx = *mx;
            }
            Instruction::Mem(addr, value) => {
                let addr = addr | mask1;
                set(&mut mem, addr, maskx, *value)
            }
        }
    }

    let sum: u64 = mem.into_iter().map(|(_key, value)| value).sum();
    println!("{}", sum);
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut prog = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let instruction = parse(&line);
        prog.push(instruction)
    }

    part1(&prog);
    part2(&prog);
}
