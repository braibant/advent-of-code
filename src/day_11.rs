use crate::intcode;
use std::collections::HashSet;

#[derive(Clone)]
struct T {
    x: i32,
    y: i32,
    dir: i32,
    white: HashSet<(i32, i32)>,
    painted: HashSet<(i32, i32)>,
}

// Execute the painting instructions
fn paint(t: &T, program: &intcode::Program) -> T {
    let mut vm = intcode::T::new(program);
    let mut t = t.clone();
    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    while !vm.is_halted() {
        let input = if t.white.contains(&(t.x, t.y)) { 1 } else { 0 };
        vm.push(input);
        let paint = vm.pop().unwrap(); // 0 means paint black, 1 means paint white
        let turn = vm.pop().unwrap(); // 0 means turn left 90 degrees, 1 means turn right 90 degrees.
        t.painted.insert((t.x, t.y));
        if paint == 0 {
            t.white.remove(&(t.x, t.y));
        } else {
            t.white.insert((t.x, t.y));
        };
        if turn == 0 {
            t.dir -= 1
        } else {
            t.dir += 1
        };
        let (dx, dy) = dirs[t.dir.rem_euclid(4) as usize];
        t.x += dx;
        t.y += dy;
    }
    t
}

impl T {
    fn new() -> T {
        T {
            x: 0,
            y: 0,
            dir: 0,
            white: HashSet::new(),
            painted: HashSet::new(),
        }
    }
}

// Column major matrix
fn matrix<T>(width: usize, height: usize, elt: T) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut t = vec![];
    for _i in 0..width + 1 {
        let mut v = vec![];
        v.resize(height + 1, elt);
        t.push(v)
    }
    t
}

// TODO: move to mini bitmap library
fn raster(pts: HashSet<(i32, i32)>) -> Vec<Vec<bool>> {
    let minx = pts.iter().map(|(x, _)| x).min().unwrap();
    let maxx = pts.iter().map(|(x, _)| x).max().unwrap();
    let miny = pts.iter().map(|(_, y)| y).min().unwrap();
    let maxy = pts.iter().map(|(_, y)| y).max().unwrap();

    let width = (maxx - minx) as usize;
    let height = (maxy - miny) as usize;
    let mut m = matrix(width, height, false);

    for (x, y) in pts.iter() {
        m[(x - minx) as usize][(y - miny) as usize] = true
    }
    m
}

// TODO: move to mini bitmap library
fn print(m: &Vec<Vec<bool>>) {
    for line in m.iter() {
        for &c in line.iter() {
            if c {
                print!("X")
            } else {
                print!(" ")
            }
        }
        println!("")
    }
}

pub fn run(filename: String) {
    let program = intcode::read_intcode_program(&filename);
    let t = T::new();
    let part1 = paint(&t, &program);
    println!("{}", part1.painted.len());

    let mut t = T::new();
    t.white.insert((0, 0));
    let part2 = paint(&t, &program);
    print(&raster(part2.white))
}
