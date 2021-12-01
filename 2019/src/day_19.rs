use crate::intcode;
use std::collections::HashMap;

fn part1(program: &intcode::Program) -> u64 {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut vm = intcode::T::new(&program);
            vm.push(x);
            vm.push(y);
            let out = vm.get_output().unwrap();
            if out == 0 {
                print!(".")
            } else if out == 1 {
                print!("#");
                count += 1
            } else {
                panic!("Unexpected intcode output {}", out)
            }
        }
        println!("")
    }
    count
}

// The program that we are given denotes a region of the 2d plane (the beam).
// The goal of part 2 is to find a place in this beam where we can fit a given
// shape (a square of size 100). The beam grows wider as it goes further away
// from the emitter located in position 0,0.
// 1. One observation is that the bottom left corner of the square must be on
//    the edge of the beam (horizontally). (otherwise, we could move the square one position to
//    the left, and it would still fit inside the beam).
// 2. A second observation is that the top right corner of the square must be on
//    the edge of the beam (vertically).

struct T {
    program: Vec<i64>,
    cache: HashMap<(u32, u32), bool>,
}

impl T {
    fn new(program: &[i64]) -> T {
        let program = program.to_vec();
        T {
            program,
            cache: HashMap::new(),
        }
    }
}

fn inside_beam(t: &mut T, x: u32, y: u32) -> bool {
    let program = &t.program;
    let cache = &mut t.cache;
    let entry = cache.entry((x, y)).or_insert_with(|| {
        let mut vm = intcode::T::new(program);
        vm.push(x as i64);
        vm.push(y as i64);
        let out = vm.get_output().unwrap();
        match out {
            1 => true,
            0 => false,
            _ => panic!("Unexpected output"),
        }
    });
    *entry
}

// Under the assumption that f(l) != f(r), finds m such that f(l) =
// f(m), and f(m+1) = f(r).
fn bs<F>(mut f: F, mut l: u32, mut r: u32) -> Option<u32>
where
    F: FnMut(u32) -> bool,
{
    let mut bl = f(l);
    let mut br = f(r);
    assert!(bl != br);
    while bl != br && l + 1 < r {
        let m = l + (r - l) / 2;
        let bm = f(m);
        if bm == bl {
            bl = bm;
            l = m
        } else {
            br = bm;
            r = m
        }
    }
    if bl != br && l + 1 == r {
        Some(l)
    } else {
        None
    }
}

fn bs_edge(t: &mut T, y: u32, l: u32, r: u32) -> Option<u32> {
    bs(|x| inside_beam(t, x, y), l, r)
}

// Return the xmin such that inside_beam(x, y) and the xmax such that inside_beam(x,y)
fn edges(t: &mut T, slope: f64, y: u32) -> Option<(u32, u32)> {
    let beam = (slope * (y as f64)) as u32;
    let l = bs_edge(t, y, 0, beam)?;
    let r = bs_edge(t, y, beam, 3 * beam)?;
    Some((l + 1, r))
}

fn find_slope(t: &mut T, y: u32) -> Option<f64> {
    let mut xmin = None;
    let mut xmax = None;
    for x in 0..(y * 2) {
        if inside_beam(t, x, y) && !(inside_beam(t, x + 1, y)) {
            xmax = Some(x)
        };
        if !inside_beam(t, x, y) && inside_beam(t, x + 1, y) {
            xmin = Some(x + 1)
        }
    }
    let xmin = xmin?;
    let xmax = xmax?;
    let mid = ((xmin as f64) + (xmax as f64)) / 2.0;
    Some(mid / (y as f64))
}

fn valid_position(t: &mut T, slope: f64, y: u32, size: u32) -> bool {
    let (xmin, xmax) = edges(t, slope, y).unwrap();
    xmin + (size - 1) <= xmax && inside_beam(t, xmin + (size - 1), y - (size - 1))
}

fn square_best_fit(program: &intcode::Program, size: u32) -> (u32, u32) {
    let mut t = T::new(program);
    // Let's find the slope of a ray inside the beam, far away from the emitter.
    // To do that, we can enumerate the positions with y = 1000, take the middle
    // of the beam, and compute the slope. Once we have the slope, for a given
    // y, we can find the left edge via binary search (call this position x),
    // and check if (x+100, y - 100) is inside the beam. If yes, we can check if
    // this is the first such position (maybe, via binary search). If not,
    // increase y (maybe, by doubling y)
    let slope = find_slope(&mut t, 500).unwrap();
    let mut y = size;
    while !valid_position(&mut t, slope, y, size) {
        y *= 2;
    }
    let y = bs(|y| valid_position(&mut t, slope, y, size), y / 2, y).unwrap();
    let (xmin, _) = edges(&mut t, slope, y).unwrap();
    (xmin, y - (size - 1))
}

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(filename);
    // x ->
    // y v
    println!("{}", part1(&program));
    let (x, y) = square_best_fit(&program, 100);
    println!("{}", x * 10_000 + y);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(dead_code)]
    fn test_edges(program: &[i64], y: u32, size: u32) {
        let mut t = T::new(program);
        let min = (0..size)
            .filter(|&i| inside_beam(&mut t, i, y))
            .min()
            .unwrap();
        let max = (0..size)
            .filter(|&i| inside_beam(&mut t, i, y))
            .max()
            .unwrap();
        let slope = ((max + min) as f64) / ((2 * y) as f64);
        let (xmin, xmax) = edges(&mut t, slope, y).unwrap();
        assert_eq!(min, xmin);
        assert_eq!(max, xmax);
    }
}
