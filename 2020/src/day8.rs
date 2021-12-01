use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl Op {
    fn parse(s: &str) -> Op {
        match s {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            s => panic!("Invalid instruction {}", s),
        }
    }
}

fn parse(s: &str) -> (Op, i32) {
    let s: Vec<&str> = s.split(" ").collect();
    assert_eq!(s.len(), 2);
    let op = Op::parse(s[0]);
    let arg: i32 = s[1].parse().unwrap();
    return (op, arg);
}

enum Execution {
    Loop(i32),
    Terminate(i32),
    Fault(i32, i32),
}

fn execute_until_loop(prog: &Vec<(Op, i32)>) -> Execution {
    let mut pc: i32 = 0;
    let mut acc: i32 = 0;
    let mut trace: Vec<bool> = Vec::new();
    trace.resize(prog.len(), false);
    loop {
        if pc as usize == prog.len() {
            return Execution::Terminate(acc);
        } else if trace[pc as usize] {
            return Execution::Loop(acc);
        } else {
            trace[pc as usize] = true;
            let (op, arg) = &prog[pc as usize];
            match op {
                Op::Nop => pc += 1,
                Op::Acc => {
                    acc += arg;
                    pc += 1
                }
                Op::Jmp => {
                    if prog.len() < (pc as usize) {
                        return Execution::Fault(pc, acc);
                    } else {
                        pc += arg
                    }
                }
            }
        }
    }
}

fn flip_nop_jmp(prog: &mut Vec<(Op, i32)>, i: usize) {
    match prog[i] {
        (Op::Nop, arg) => prog[i] = (Op::Jmp, arg),
        (Op::Jmp, arg) => prog[i] = (Op::Nop, arg),
        _ => {}
    }
}

// Input contain lines of the form `name -> (int * name) list` (the actual input is a bit obfuscated).
pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut prog: Vec<(Op, i32)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        prog.push(parse(&line));
    }

    // part 1
    match execute_until_loop(&prog) {
        Execution::Loop(acc) => println!("{}", acc),
        _ => panic!("Unexpected"),
    };
    // part 2
    for i in 0..prog.len() {
        flip_nop_jmp(&mut prog, i);
        match execute_until_loop(&prog) {
            Execution::Terminate(acc) => println!("{}", acc),
            _ => {}
        }
        flip_nop_jmp(&mut prog, i);
    }
}
