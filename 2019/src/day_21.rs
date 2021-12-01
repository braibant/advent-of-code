use crate::intcode;
// use minisat;
// use minisat::symbolic::*;
use std::collections::HashSet;
use z3::ast::{Ast, Bool};

fn test(program: &[i64], input: &str) -> Result<i64, String> {
    let mut vm = intcode::T::new(program);
    let mut output = String::new();
    vm.execute();
    vm.push_str(input);
    while vm.outputs() > 0 {
        let c = vm.get_output().unwrap();
        if 0 <= c && c < 255 {
            output.push((c as u8) as char);
        } else {
            return Ok(c);
        }
    }
    return Err(output);
}

const PART1: &'static str = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
WALK
";

fn part1(program: &[i64]) {
    let output = test(program, &PART1).unwrap();
    println!("{}", output);
}

struct T<'ctx> {
    ctx: &'ctx z3::Context,
    solver: z3::Solver<'ctx>,
    instruction: z3::DatatypeSort<'ctx>,
    registers: usize,
    // INSTRUCTION REGISTER REGISTER
    program: Vec<(
        z3::ast::Datatype<'ctx>,
        z3::ast::Int<'ctx>,
        z3::ast::Int<'ctx>,
    )>,
    samples: HashSet<String>,
    c_t: z3::ast::Int<'ctx>,
    c_j: z3::ast::Int<'ctx>,
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Instruction {
    AND,
    OR,
    NOT,
}

impl<'ctx> T<'ctx> {
    fn new(ctx: &'ctx z3::Context, instructions: usize, registers: usize) -> T<'ctx> {
        let solver = z3::Solver::new(ctx);
        let instruction = z3::DatatypeBuilder::new(&ctx, "INSTRUCTION")
            .variant("OR", vec![])
            .variant("AND", vec![])
            .variant("NOT", vec![])
            .finish();
        let program: Vec<_> = (0..instructions)
            .map(|i| {
                let i =
                    z3::ast::Datatype::new_const(ctx, format!("instr_{}", i), &instruction.sort);
                let lhs = z3::ast::Int::new_const(ctx, format!("lhs_{}", i));
                let rhs = z3::ast::Int::new_const(ctx, format!("rhs_{}", i));
                (i, lhs, rhs)
            })
            .collect();
        let c_zero = z3::ast::Int::from_u64(ctx, 0);
        let c_registers = z3::ast::Int::from_u64(ctx, (registers + 1) as u64);
        let c_t = z3::ast::Int::from_u64(ctx, registers as u64);
        let c_j = z3::ast::Int::from_u64(ctx, (registers + 1) as u64);

        for (instr, lhs, rhs) in program.iter() {
            solver.assert(&c_zero.le(lhs));
            solver.assert(&lhs.le(&c_registers));
            solver.assert(&c_t.le(rhs));
            solver.assert(&rhs.le(&c_j));
        }
        T {
            ctx: &ctx,
            solver,
            instruction,
            program,
            registers,
            samples: HashSet::new(),
            c_t,
            c_j,
        }
    }

    fn eval(&mut self, sensors: &[bool]) -> z3::ast::Bool<'ctx> {
        let mut t = z3::ast::Bool::from_bool(self.ctx, false);
        let mut j = z3::ast::Bool::from_bool(self.ctx, false);

        let sensors: Vec<_> = sensors
            .iter()
            .map(|&b| z3::ast::Bool::from_bool(self.ctx, b))
            .collect();
        for (i, (instr, lhs, rhs)) in self.program.iter().enumerate() {
            let input_lhs = z3::ast::Bool::fresh_const(self.ctx, &format!("input_lhs_{}", i));
            let input_rhs = z3::ast::Bool::fresh_const(self.ctx, &format!("input_rhs_{}", i));
            let output = z3::ast::Bool::fresh_const(self.ctx, &format!("output_{}", i));

            // Build input lhs
            for (reg, content) in sensors.iter().enumerate() {
                let c_r = z3::ast::Int::from_u64(self.ctx, reg as u64);
                self.solver
                    .assert(&lhs._eq(&c_r).implies(&input_lhs._eq(&content)))
            }

            self.solver
                .assert(&lhs._eq(&self.c_t).implies(&input_lhs._eq(&t)));
            self.solver
                .assert(&lhs._eq(&self.c_j).implies(&input_lhs._eq(&j)));

            // Build input rhs
            self.solver.assert(
                &rhs._eq(&self.c_t)
                    .ite(&input_rhs._eq(&t), &input_rhs._eq(&j)),
            );

            // Build output
            // OR = 0
            let is_or = (self.instruction.variants[0]
                .tester
                .apply(&[instr])
                .as_bool()
                .unwrap());
            self.solver.assert(
                &is_or
                    .implies(&output._eq(&z3::ast::Bool::or(self.ctx, &[&input_lhs, &input_rhs]))),
            );

            // AND = 1
            let is_and = (self.instruction.variants[1]
                .tester
                .apply(&[instr])
                .as_bool()
                .unwrap());
            self.solver
                .assert(&is_and.implies(
                    &output._eq(&z3::ast::Bool::and(self.ctx, &[&input_lhs, &input_rhs])),
                ));

            // NOT = 2
            let is_not = (self.instruction.variants[2]
                .tester
                .apply(&[instr])
                .as_bool()
                .unwrap());
            self.solver
                .assert(&is_not.implies(&output._eq(&input_lhs.not())));

            // Update T / J to new values
            t = rhs._eq(&self.c_t).ite(&output, &t);
            j = rhs._eq(&self.c_j).ite(&output, &j);
        }
        j
    }

    fn encode(&mut self, sample: &[char]) {
        let mut jumped = vec![];
        for start in 0..sample.len() {
            let mut in_the_air = vec![];
            for i in start.saturating_sub(3)..start {
                in_the_air.push(&jumped[i])
            }
            let in_the_air = z3::ast::Bool::or(self.ctx, &in_the_air);
            if sample[start] != '#' {
                self.solver.assert(&in_the_air)
            }
            let sensors: Vec<_> = sample
                [(start + 1)..std::cmp::min(sample.len(), start + self.registers + 1)]
                .iter()
                .map(|&c| c == '#')
                .collect();
            let jump = self.eval(&sensors);
            let jump = z3::ast::Bool::and(self.ctx, &[&jump, &in_the_air.not()]);
            jumped.push(jump)
        }
    }

    fn decode(&self, verb: &str) -> String {
        let mut buf = String::new();

        if self.solver.check() != z3::SatResult::Sat {
            panic!("Failed to decode")
        };

        let model = self.solver.get_model().unwrap();

        for (instr, lhs, rhs) in self.program.iter() {
            let is_or = model
                .eval(
                    &self.instruction.variants[0]
                        .tester
                        .apply(&[instr])
                        .as_bool()
                        .unwrap(),
                    true,
                )
                .unwrap()
                .as_bool()
                .unwrap();
            let is_and = model
                .eval(
                    &self.instruction.variants[1]
                        .tester
                        .apply(&[instr])
                        .as_bool()
                        .unwrap(),
                    true,
                )
                .unwrap()
                .as_bool()
                .unwrap();
            let is_not = model
                .eval(
                    &self.instruction.variants[2]
                        .tester
                        .apply(&[instr])
                        .as_bool()
                        .unwrap(),
                    true,
                )
                .unwrap()
                .as_bool()
                .unwrap();
            let instr = match (is_or, is_and, is_not) {
                (true, false, false) => Instruction::OR,
                (false, true, false) => Instruction::AND,
                (false, false, true) => Instruction::NOT,
                _ => panic!(),
            };
            let lhs = model.eval(lhs, false).unwrap().as_u64().unwrap() as usize;
            let rhs = model.eval(rhs, false).unwrap().as_u64().unwrap() as usize;
            let lhs = if lhs < self.registers {
                (b'A' + lhs as u8) as char
            } else if lhs == self.registers {
                'T'
            } else {
                'J'
            };
            let rhs = if rhs == self.registers { 'T' } else { 'J' };
            buf.push_str(&format!("{:?} {} {}\n", instr, lhs, rhs));
        }
        buf.push_str(&format!("{}\n", verb));
        buf
    }
}

fn part2(program: &[i64]) {
    let config = z3::Config::new();
    let context = z3::Context::new(&config);
    let mut t = T::new(&context, 7, 9);
    loop {
        let script = t.decode("RUN");
        // println!("{}", script);
        match test(program, &script) {
            Ok(x) => {
                println!("{}", x);
                break;
            }
            Err(output) => {
                let situation: String = output.split('\n').rev().nth(2).unwrap().to_string();
                if t.samples.contains(&situation) {
                    panic!("Unable to make progress")
                } else {
                    let sensors: Vec<_> = situation.chars().collect();
                    t.samples.insert(situation);
                    t.encode(&sensors)
                }
            }
        }
    }
}

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(filename);
    part1(&program);
    part2(&program);
}

// #[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
// enum Op {
//     AND,
//     OR,
//     NOT,
// }

// #[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Copy)]
// #[repr(usize)]
// enum Register {
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     H,
//     I,
//     T,
//     J,
// }

// use lazy_static::lazy_static;

// lazy_static! {
//     static ref REGISTERS: Vec<Register> = {
//         vec![
//             Register::A,
//             Register::B,
//             Register::C,
//             Register::D,
//             Register::E,
//             Register::F,
//             Register::G,
//             Register::H,
//             Register::I,
//             Register::T,
//             Register::J,
//         ]
//     };
// }

// struct T<'ctx> {
//     ninstructions: usize,
//     nsensors: usize,
//     registers: Vec<Register>,
//     sensors: Vec<z3::ast::Bool<'ctx>>,
//     instructions: Vec<minisat::symbolic::Symbolic<Op>>,
//     lhs: Vec<minisat::symbolic::Symbolic<Register>>,
//     rhs: Vec<minisat::symbolic::Symbolic<Register>>,
//     t: Vec<z3::ast::Bool<'ctx>>,
//     j: Vec<z3::ast::Bool<'ctx>>,
// }

// impl T {
//     fn new(ninstructions: usize, nsensors: usize) -> (T, minisat::Solver) {
//         let mut state = minisat::Solver::new();
//         let sensors: Vec<_> = (0..nsensors).map(|_| state.new_lit()).collect();
//         let instructions = (0..ninstructions)
//             .map(|_i| Symbolic::new(&mut state, vec![Op::AND, Op::OR, Op::NOT]))
//             .collect::<Vec<_>>();
//         let registers: Vec<_> = REGISTERS
//             .to_vec()
//             .iter()
//             .enumerate()
//             .filter(|(i, r)| *i < nsensors || **r == Register::T || **r == Register::J)
//             .map(|(_, r)| *r)
//             .collect();
//         let lhs = (0..ninstructions)
//             .map(|_i| Symbolic::new(&mut state, registers.clone()))
//             .collect::<Vec<_>>();
//         let rhs = (0..ninstructions)
//             .map(|_i| Symbolic::new(&mut state, vec![Register::T, Register::J]))
//             .collect::<Vec<_>>();
//         let t: Vec<_> = (0..ninstructions + 1).map(|_| state.new_lit()).collect();
//         let j: Vec<_> = (0..ninstructions + 1).map(|_| state.new_lit()).collect();
//         // T / J are initialized to false
//         state.equal(&t[0], &minisat::Bool::Const(false));
//         state.equal(&j[0], &minisat::Bool::Const(false));
//         (
//             T {
//                 ninstructions,
//                 nsensors,
//                 registers,
//                 sensors,
//                 instructions,
//                 lhs,
//                 rhs,
//                 t,
//                 j,
//             },
//             state,
//         )
//     }

//     fn semantics(&self, s: &mut minisat::Solver) {
//         for i in 1..(self.ninstructions + 1) {
//             // At each step, we update one register and leave the other untouched.
//             let updated = s.new_lit();
//             let pre = s.new_lit();
//             // Define the update behavior for both registers. After this step, we
//             // only need to define the semantics of `updated` in terms of `pre`
//             // s.add_clause([s.implies(
//             //     rhs[i - 1].has_value(&Register::T),
//             //     s.and_literal([
//             //         s.equiv(t[i], updated),
//             //         s.equiv(t[i - 1], pre),
//             //         s.equiv(j[i], j[i - 1]),
//             //     ]),
//             // )]);

//             // if RHS is T
//             let v = vec![
//                 s.equiv(self.t[i], updated),
//                 s.equiv(self.t[i - 1], pre),
//                 s.equiv(self.j[i], self.j[i - 1]),
//             ];
//             let v = s.and_literal(v);
//             let v = s.implies(self.rhs[i - 1].has_value(&Register::T), v);
//             s.add_clause([v]);

//             // if RHS is J
//             let v = vec![
//                 s.equiv(self.j[i], updated),
//                 s.equiv(self.j[i - 1], pre),
//                 s.equiv(self.t[i], self.t[i - 1]),
//             ];
//             let v = s.and_literal(v);
//             let v = s.implies(self.rhs[i - 1].has_value(&Register::J), v);
//             s.add_clause([v]);

//             // Now, we define the actual value for the LHS of this instruction. It's
//             // either a sensor, or the previous value of a register
//             let actual = s.new_lit();
//             let regs = self
//                 .registers
//                 .iter()
//                 .enumerate()
//                 .map(|(id, r)| {
//                     let is_matching = self.lhs[i - 1].has_value(r);
//                     let value = match r {
//                         Register::T => self.t[i - 1],
//                         Register::J => self.j[i - 1],
//                         _ => self.sensors[id],
//                     };
//                     let equiv = s.equiv(value, actual);
//                     s.implies(is_matching, equiv)
//                 })
//                 .collect::<Vec<_>>();
//             let regs = s.and_literal(regs);
//             s.add_clause([regs]);

//             // We now move to defining the actual semantics of the instruction.
//             let andl = s.and_literal([actual, pre]);
//             let equiv_andl = s.equiv(updated, andl);
//             let and_impl = s.implies(self.instructions[i - 1].has_value(&Op::AND), equiv_andl);

//             let orl = s.or_literal([actual, pre]);
//             let equiv_orl = s.equiv(updated, orl);
//             let or_impl = s.implies(self.instructions[i - 1].has_value(&Op::OR), equiv_orl);

//             let equiv_notl = s.equiv(updated, !actual);
//             let not_impl = s.implies(self.instructions[i - 1].has_value(&Op::NOT), equiv_notl);

//             let all = s.and_literal([and_impl, or_impl, not_impl]);
//             s.add_clause([all]);
//         }
//     }

//     fn encode_sample(&self, s: &mut minisat::Solver, sample: &[char]) -> Vec<minisat::Bool> {
//         let mut jumped = vec![];

//         for start in 0..sample.len() {
//             let mut in_the_air = vec![];
//             for i in start.saturating_sub(3)..start {
//                 in_the_air.push(jumped[i])
//             }
//             let in_the_air = s.or_literal(in_the_air);
//             if sample[start] != '#' {
//                 s.add_clause([in_the_air]);
//             }

//             let mut eval_in = vec![];
//             let mut sensor = 0;
//             while sensor < self.nsensors && sensor + start + 1 < sample.len() {
//                 eval_in.push(s.equiv(
//                     self.sensors[sensor],
//                     minisat::Bool::Const(sample[sensor + start + 1] == '#'),
//                 ));
//                 sensor += 1
//             }
//             let eval_in = s.and_literal(eval_in);
//             let jump = s.new_lit();
//             let eval_out = s.equiv(jump, self.j[self.ninstructions]);
//             let eval = s.implies(eval_in, eval_out);
//             s.add_clause([eval]);
//             jumped.push(s.and_literal([jump, !in_the_air]));
//         }
//         jumped
//     }
// }

// // Let's encode the program as a SAT problem, use each failed run to create new
// // situations.
// fn encode(
//     program: &[i64],
//     ninstructions: usize,
//     nsensors: usize,
//     verb: &str,
//     samples: &HashSet<Vec<char>>,
// ) -> Result<(i64, String), Vec<char>> {
//     let (vars, mut s) = T::new(ninstructions, nsensors);
//     vars.semantics(&mut s);

//     // Now, we turn to encode the samples.
//     for sample in samples.iter() {
//         let _ = vars.encode_sample(&mut s, &sample);
//     }
//     println!(
//         "Vars: {}, clauses: {}, samples: {}",
//         s.num_vars(),
//         s.num_clauses(),
//         samples.len()
//     );
//     let model = s.solve().unwrap();
//     let mut source = String::new();
//     for i in 0..ninstructions {
//         let instruction = model.value(&vars.instructions[i]);
//         let lhs = model.value(&vars.lhs[i]);
//         let rhs = model.value(&vars.rhs[i]);
//         source.push_str(&format!("{:?} {:?} {:?}\n", instruction, lhs, rhs));
//     }
//     source.push_str(&format!("{}\n", verb));

//     println!("{}", source);
//     match test(program, &source) {
//         Ok(i) => return Ok((i, source)),
//         Err(output) => {
//             println!("{}", output);
//             let situation: Vec<_> = output.split('\n').rev().nth(2).unwrap().chars().collect();
//             Err(situation)
//         }
//     }
// }

// fn part2(program: &[i64]) {
//     let mut samples = HashSet::new();
//     samples.insert("#####.###########".chars().collect());
//     samples.insert("#####...#########".chars().collect());
//     loop {
//         match encode(program, 10, 4, "WALK", &samples) {
//             Ok((i, source)) => break,
//             Err(sample) => {
//                 println!("{:?}", sample);
//                 if !samples.insert(sample) {
//                     panic!()
//                 };
//             }
//         }
//     }
// }

// pub fn run(filename: &str) {
//     let program = intcode::read_intcode_program(filename);
//     part1(&program);
//     part2(&program);
// }

// fn eval(program: &[(Op, Register, Register)], scan: &[bool]) -> bool {
//     let mut t = false;
//     let mut j = false;
//     for (op, lhs, rhs) in program.iter() {
//         let lhs = match lhs {
//             Register::T => t,
//             Register::J => j,
//             &x => scan[x as usize],
//         };
//         let (value, tgt) = match rhs {
//             // It's fascinating to me that rust bakes some left to right
//             // evaluation order in the following expression, and that the
//             // compilers rejects (&mut t, t) (which could be let expanded.)
//             Register::T => (t, &mut t),
//             Register::J => (j, &mut j),
//             _ => panic!(),
//         };
//         *tgt = match op {
//             Op::AND => value && lhs,
//             Op::OR => value || lhs,
//             Op::NOT => !lhs,
//         }
//     }
//     j
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn set<T>(s: &mut minisat::Solver, l: &minisat::symbolic::Symbolic<T>, t: &T)
//     where
//         T: Eq,
//     {
//         //    s.equal(&l.has_value(t), &minisat::Bool::Const(true))
//         s.add_clause([l.has_value(t)])
//     }

//     fn check_semantics(program: &[(Op, Register, Register)], scan: &[bool]) {
//         let (t, mut s) = T::new(program.len(), scan.len());
//         t.semantics(&mut s);
//         for (i, (op, lhs, rhs)) in program.iter().enumerate() {
//             set(&mut s, &t.instructions[i], op);
//             set(&mut s, &t.lhs[i], lhs);
//             set(&mut s, &t.rhs[i], rhs);
//         }

//         for (i, v) in scan.iter().enumerate() {
//             s.equal(&t.sensors[i], &minisat::Bool::Const(*v))
//         }

//         let actual = eval(program, scan);
//         let m = s.solve_under_assumptions([]).unwrap();
//         assert_eq!(m.value(&t.j[t.ninstructions]), actual);
//         let j = s.equiv(t.j[t.ninstructions], minisat::Bool::Const(!actual));
//         let m = s.solve_under_assumptions([j]);
//         assert!(m.is_err())
//     }

//     #[test]
//     fn test_semantics() {
//         let program = vec![
//             (Op::NOT, Register::A, Register::J),
//             (Op::AND, Register::D, Register::J),
//         ];
//         let scan = vec![false, true, true, true, false];
//         check_semantics(&program, &scan);
//     }

//     #[test]
//     fn test_1() {
//         let (t, mut s) = T::new(1, 9);
//         let _true = minisat::Bool::Const(true);
//         let _false = minisat::Bool::Const(false);
//         t.semantics(&mut s);
//         set(&mut s, &t.instructions[0], &Op::NOT);
//         set(&mut s, &t.lhs[0], &Register::A);
//         set(&mut s, &t.rhs[0], &Register::J);

//         // A false
//         let a = s.equiv(t.sensors[0], _false);
//         let m = s.solve_under_assumptions([a]).unwrap();
//         assert_eq!(m.value(&t.j[1]), true);
//         assert_eq!(m.value(&t.t[1]), false);
//         let j = s.equiv(t.j[1], _false);
//         let m = s.solve_under_assumptions([a, j]);
//         assert!(m.is_err());

//         // A true
//         let a = s.equiv(t.sensors[0], _true);
//         let m = s.solve_under_assumptions([a]).unwrap();
//         assert_eq!(m.value(&t.j[1]), false);
//         assert_eq!(m.value(&t.t[1]), false);
//         let j = s.equiv(t.j[1], _true);
//         let m = s.solve_under_assumptions([a, j]);
//         assert!(m.is_err());
//     }

//     #[test]
//     fn test_2() {
//         let (t, mut s) = T::new(2, 9);
//         let _true = minisat::Bool::Const(true);
//         let _false = minisat::Bool::Const(false);
//         t.semantics(&mut s);
//         set(&mut s, &t.instructions[0], &Op::NOT);
//         set(&mut s, &t.lhs[0], &Register::A);
//         set(&mut s, &t.rhs[0], &Register::J);

//         set(&mut s, &t.instructions[1], &Op::AND);
//         set(&mut s, &t.lhs[1], &Register::D);
//         set(&mut s, &t.rhs[1], &Register::J);

//         // A, D false
//         let a = s.equiv(t.sensors[0], _false);
//         let d = s.equiv(t.sensors[3], _false);
//         let m = s.solve_under_assumptions([a, d]).unwrap();
//         assert_eq!(t.ninstructions, 2);
//         assert_eq!(m.value(&t.j[2]), false);
//         let j = s.equiv(t.j[2], _true);
//         let m = s.solve_under_assumptions([a, d, j]);
//         assert!(m.is_err());

//         // A false, D true
//         let a = s.equiv(t.sensors[0], _false);
//         let d = s.equiv(t.sensors[3], _true);
//         let m = s.solve_under_assumptions([a, d]).unwrap();
//         assert_eq!(m.value(&t.j[2]), true);
//         let j = s.equiv(t.j[2], _false);
//         let m = s.solve_under_assumptions([a, d, j]);
//         assert!(m.is_err());
//     }

//     #[test]
//     fn test_sample_1() {
//         let (t, mut s) = T::new(5, 2);
//         let _true = minisat::Bool::Const(true);
//         let _false = minisat::Bool::Const(false);
//         t.semantics(&mut s);

//         let scan = vec!['#', '.', '#', '#', '#', '#', '#', '#'];
//         let jumped = t.encode_sample(&mut s, &scan);
//         let a = s.equiv(t.sensors[0], _false);
//         let b = s.equiv(t.sensors[1], _true);
//         let j = s.equiv(t.j[t.ninstructions], _false);
//         let m = s.solve_under_assumptions([a, b, j]);
//         assert!(m.is_err());
//     }

//     #[test]
//     fn test_sample_2() {
//         let (t, mut s) = T::new(5, 2);
//         let _true = minisat::Bool::Const(true);
//         let _false = minisat::Bool::Const(false);
//         t.semantics(&mut s);

//         let scan = vec!['#', '.', '#', '#', '#', '#', '#', '#'];
//         let jumped1 = t.encode_sample(&mut s, &scan);
//         let scan = vec!['#', '.', '.', '#', '#', '#', '#', '#'];
//         let jumped1 = t.encode_sample(&mut s, &scan);
//         let a = s.equiv(t.sensors[0], _false);
//         let b = s.equiv(t.sensors[1], _true);
//         let j = s.equiv(t.j[t.ninstructions], _false);
//         let m = s.solve_under_assumptions([a, b, j]);

//         assert!(m.is_err());

//         let a = s.equiv(t.sensors[0], _false);
//         let b = s.equiv(t.sensors[1], _false);
//         let j = s.equiv(t.j[t.ninstructions], _false);
//         let m = s.solve_under_assumptions([a, b, j]);
//         assert!(m.is_err());

//         let a = s.equiv(t.sensors[0], _false);
//         let m = s.solve_under_assumptions([a]).unwrap();
//         assert_eq!(m.value(&t.j[t.ninstructions]), true);
//     }
//     #[test]
//     fn test_sample_3() {
//         let (t, mut s) = T::new(10, 4);
//         let _true = minisat::Bool::Const(true);
//         let _false = minisat::Bool::Const(false);
//         t.semantics(&mut s);

//         let scan = vec!['#', '.', '.', '.', '#', '#', '#', '#'];
//         let jumped1 = t.encode_sample(&mut s, &scan);
//         let a = s.equiv(t.sensors[0], _false);
//         let b = s.equiv(t.sensors[1], _false);
//         let c = s.equiv(t.sensors[2], _false);
//         let d = s.equiv(t.sensors[3], _true);
//         let j = s.equiv(t.j[t.ninstructions], _false);
//         let m = s.solve_under_assumptions([a, b, c, d, j]);

//         assert!(m.is_err());

//         let j = s.equiv(t.j[t.ninstructions], _true);
//         let m = s.solve_under_assumptions([a, b, c, d, j]);

//         assert!(!m.is_err());
//     }

//     #[test]
//     fn test_sample_4() {
//         let (t, mut s) = T::new(10, 4);
//         let _true = minisat::Bool::Const(true);
//         let _false = minisat::Bool::Const(false);
//         t.semantics(&mut s);

//         let scan = vec!['#', '#', '.', '.', '.', '#', '#', '#', '#'];
//         let jumped1 = t.encode_sample(&mut s, &scan);
//         let a = s.equiv(t.sensors[0], _false);
//         let b = s.equiv(t.sensors[1], _false);
//         let c = s.equiv(t.sensors[2], _false);
//         let d = s.equiv(t.sensors[3], _true);
//         let j = s.equiv(t.j[t.ninstructions], _false);
//         let m = s.solve_under_assumptions([a, b, c, d, j]);
//         assert!(m.is_err());
//         let j = s.equiv(t.j[t.ninstructions], _true);
//         let m = s.solve_under_assumptions([a, b, c, d, j]);
//         assert!(!m.is_err());
//     }
// }
