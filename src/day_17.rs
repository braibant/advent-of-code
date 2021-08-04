use crate::intcode;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    R,
    L,
    F(u8),
    A,
    B,
    C,
}

#[derive(Debug)]
struct Input {
    commands: Vec<Instruction>,
    a: Vec<Instruction>,
    b: Vec<Instruction>,
    c: Vec<Instruction>,
}

#[derive(Clone, Debug)]
struct Problem {
    scaffold: HashSet<(i8, i8)>,
    dir: i8,
    start: (i8, i8),
}

use lazy_static::lazy_static;

lazy_static! {
    // N, E, S, W. (0,0) is bottom left of screen.
    static ref DIRS: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
}

fn is_path_complete(problem: &Problem, instructions: &[Instruction]) -> bool {
    let mut dir = problem.dir;
    let mut x = problem.start.0;
    let mut y = problem.start.1;
    let mut visited = HashSet::with_capacity(problem.scaffold.len());
    for i in instructions.iter() {
        match i {
            Instruction::L => dir = (dir - 1).rem_euclid(4),
            Instruction::R => dir = (dir + 1).rem_euclid(4),
            Instruction::F(n) => {
                let (dx, dy) = DIRS[dir as usize];
                for _i in 0..*n {
                    x += dx;
                    y += dy;
                    if !problem.scaffold.contains(&(x, y)) {
                        return false;
                    }
                    visited.insert((x, y));
                }
            }
            _ => panic!("Unexpected instruction"),
        }
    }
    problem.scaffold.eq(&visited)
}

fn instruction_to_u8(c: Instruction) -> Vec<u8> {
    match c {
        Instruction::L => vec![b'L'],
        Instruction::R => vec![b'R'],
        Instruction::F(n) => {
            let mut n = n;
            let mut acc = vec![];
            while n > 0 {
                acc.push(b'0' + (n % 10));
                n /= 10;
            }
            acc.reverse();
            acc
        }
        Instruction::A => vec![b'A'],
        Instruction::B => vec![b'B'],
        Instruction::C => vec![b'C'],
    }
}

fn push_instructions(vm: &mut intcode::T, instructions: &[Instruction]) {
    let mut acc = vec![];
    for i in 0..(instructions.len() - 1) {
        acc.extend_from_slice(&instruction_to_u8(instructions[i]));
        acc.push(b',');
    }
    acc.extend_from_slice(&instruction_to_u8(instructions[instructions.len() - 1]));
    acc.push(b'\n');
    acc.iter().for_each(|&e| vm.push_u8(e));
}

fn supply_input(vm: &mut intcode::T, input: &Input, feed: u8) {
    push_instructions(vm, &input.commands);
    push_instructions(vm, &input.a);
    push_instructions(vm, &input.b);
    push_instructions(vm, &input.c);
    vm.push_u8(feed);
    vm.push_u8(b'\n')
}

fn print(vm: &mut intcode::T) {
    vm.execute();
    while !vm.is_halted() || !vm.output.is_empty() {
        let c = vm.pop().unwrap();
        if c > 255 {
            eprintln!("Non ASCII value: {}", c);
        } else {
            let c = (c as u8) as char;
            print!("{}", c)
        }
    }
}

struct Move {
    dx: i8,
    dy: i8,
    dd: i8,
    is: Vec<Instruction>,
}

// We could tabulate this for the 4 values of d that we care about.
fn moves(d: i8) -> Vec<Move> {
    let mut acc = vec![];
    for &dd in vec![0, -1, 1].iter() {
        let d = (d + dd).rem_euclid(4);
        let (dx, dy) = DIRS[d as usize];
        let is = {
            if dd == -1 {
                vec![Instruction::L, Instruction::F(1)]
            } else if dd == 1 {
                vec![Instruction::R, Instruction::F(1)]
            } else {
                vec![Instruction::F(1)]
            }
        };
        acc.push(Move { dx, dy, dd, is })
    }
    acc
}

#[derive(Debug, Clone)]
struct Frame {
    x: i8,
    y: i8,
    dir: i8,
    path: Vec<Instruction>,
    visited: HashSet<(i8, i8, i8)>,
    to_visit: HashSet<(i8, i8)>,
}
// Compute a path around the scaffold, under the assumption that we do not turn
// at intersections.
fn find_path(problem: &Problem) -> Option<Vec<Instruction>> {
    let mut todo = vec![Frame {
        x: problem.start.0,
        y: problem.start.1,
        dir: problem.dir,
        path: vec![],
        visited: HashSet::new(),
        to_visit: problem.scaffold.clone(),
    }];
    while !todo.is_empty() {
        let frame = todo.pop().unwrap();
        if frame.to_visit.is_empty() {
            return Some(frame.path);
        } else {
            for mv in moves(frame.dir).into_iter() {
                let x = frame.x + mv.dx;
                let y = frame.y + mv.dy;
                let dir = (frame.dir + mv.dd).rem_euclid(4);
                if problem.scaffold.contains(&(x, y)) && !frame.visited.contains(&(x, y, dir)) {
                    let mut f = frame.clone();
                    f.x = x;
                    f.y = y;
                    f.dir = dir;
                    f.path.extend(mv.is);
                    f.visited.insert((x, y, dir));
                    f.to_visit.remove(&(x, y));
                    todo.push(f);
                    break;
                };
                // If not intersection then break.
            }
        }
    }
    None
}

fn compress(instructions: &[Instruction]) -> Vec<Instruction> {
    let mut acc = Vec::with_capacity(instructions.len());
    let mut todo = instructions[0];
    for i in instructions.iter().skip(1) {
        match (todo, i) {
            (Instruction::F(n), Instruction::F(m)) => todo = Instruction::F(m + n),
            (_, _) => {
                acc.push(todo);
                todo = *i
            }
        }
    }
    acc.push(todo);
    return acc;
}

fn collect_data(program: &intcode::Program) -> Problem {
    let mut vm = intcode::T::new(&program);
    vm.execute();
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut scaffold = HashSet::new();
    let mut dir = None;
    let mut start = None;
    while !vm.output.is_empty() {
        let c = (vm.pop().unwrap() as u8) as char;
        match c {
            '\n' => {
                y += 1;
                x = 0
            }
            '#' => {
                scaffold.insert((x, y));
                x += 1;
            }
            '^' => {
                dir = Some(0);
                start = Some((x, y))
            }
            '>' => {
                dir = Some(1);
                start = Some((x, y))
            }
            'v' => {
                dir = Some(2);
                start = Some((x, y))
            }
            '<' => {
                dir = Some(3);
                start = Some((x, y))
            }
            _ => {
                x += 1;
            }
        };
    }
    Problem {
        scaffold,
        dir: dir.unwrap(),
        start: start.unwrap(),
    }
}

fn length(instructions: &[Instruction]) -> usize {
    let instructions = compress(instructions);
    // Count the separators first
    let mut acc = instructions.len() - 1;
    for i in instructions.iter() {
        match i {
            Instruction::F(n) => {
                let mut n = *n;
                while n > 0 {
                    n /= 10;
                    acc += 1
                }
            }
            _ => acc += 1,
        }
    }
    acc
}

fn is_base(i: Instruction) -> bool {
    match i {
        Instruction::F(_) | Instruction::L | Instruction::R => true,
        _ => false,
    }
}

fn replace<T>(pattern: &[T], rhs: T, seq: &[T]) -> Vec<T>
where
    T: Clone + Eq + Copy,
{
    let mut acc: Vec<_> = seq.to_vec();
    let mut i = 0;
    while i + pattern.len() <= acc.len() {
        if acc[i..i + pattern.len()] == *pattern {
            for _i in 0..(pattern.len() - 1) {
                acc.remove(i);
            }
            acc[i] = rhs;
            i += 1
        } else {
            i += 1
        }
    }
    acc
}

fn compress_input(is: &[Instruction]) -> Option<Input> {
    for a in (1..is.len()).filter(|&a| length(&is[0..a]) <= 20) {
        let cmds_a = &is[0..a];
        let is = replace(&is[0..a], Instruction::A, is);
        let prefix_a: Vec<Instruction> = is.iter().cloned().take_while(|&i| !is_base(i)).collect();
        let is: Vec<_> = is.iter().cloned().skip_while(|&i| !is_base(i)).collect();
        for b in (1..is.len())
            .filter(|&b| length(&is[0..b]) <= 20 && is[0..b].iter().all(|&i| is_base(i)))
        {
            let cmds_b = &is[0..b];
            let is = replace(&is[0..b], Instruction::B, &is);
            let prefix_b: Vec<Instruction> =
                is.iter().cloned().take_while(|&i| !is_base(i)).collect();
            let is: Vec<_> = is.iter().cloned().skip_while(|&i| !is_base(i)).collect();
            for c in (1..is.len())
                .filter(|&c| length(&is[0..c]) <= 20 && is[0..c].iter().all(|&i| is_base(i)))
            {
                let cmds_c = &is[0..c];
                let is = replace(&is[0..c], Instruction::C, &is);
                let commands = [prefix_a.clone(), prefix_b.clone(), is.clone()].concat();
                if is.iter().all(|&i| !is_base(i)) && length(&commands) <= 20 {
                    return Some(Input {
                        commands,
                        a: compress(cmds_a),
                        b: compress(cmds_b),
                        c: compress(cmds_c),
                    });
                }
            }
        }
    }
    return None;
}

pub fn run(filename: &str) {
    let mut program = intcode::read_intcode_program(filename);
    let mut vm = intcode::T::new(&program);
    print(&mut vm);
    let problem = collect_data(&program);

    // part 1
    let is_intersection = |x, y| {
        DIRS.iter()
            .all(|(dx, dy)| problem.scaffold.contains(&(x + dx, y + dy)))
    };
    let intersections: Vec<_> = problem
        .scaffold
        .iter()
        .filter(|(x, y)| is_intersection(x, y))
        .collect();

    println!(
        "{}",
        intersections
            .iter()
            .fold(0 as i64, |acc, (x, y)| acc + (*x as i64) * (*y as i64))
    );

    // part 2

    let path = find_path(&problem).unwrap();
    assert!(is_path_complete(&problem, &path));
    let input = compress_input(&path).unwrap();
    println!("{:?}", input);
    program[0] = 2;
    let mut vm = intcode::T::new(&program);
    supply_input(&mut vm, &input, b'n');
    print(&mut vm);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace() {
        assert_eq!(replace(&['a', 'b'], 'x', &['a', 'b', 'a', 'b']), ['x', 'x']);
        assert_eq!(
            replace(&['a', 'b'], 'x', &['c', 'a', 'b', 'b']),
            ['c', 'x', 'b']
        );
        assert_eq!(
            replace(&['a', 'b', 'a'], 'x', &['a', 'b', 'a', 'b', 'a']),
            ['x', 'b', 'a']
        )
    }
}
