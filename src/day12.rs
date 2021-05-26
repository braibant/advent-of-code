use std::fs::File;
use std::io::{BufRead, BufReader};

const NORTH: (i32, i32) = (1, 0);
const SOUTH: (i32, i32) = (-1, 0);
const EAST: (i32, i32) = (0, 1);
const WEST: (i32, i32) = (0, -1);

// In this problem, the ship can face 4 directions. Rotations are a multiple of 90 degrees.
const DIRS: [(i32, i32); 4] = [NORTH, EAST, SOUTH, WEST];

#[derive(Debug)]
enum Action {
    Move((i32, i32), i32),
    Rotate(i32),
    Forward(i32),
}

fn parse(s: &str) -> Action {
    let value: i32 = s[1..].parse().unwrap();
    let action = match &s[0..1] {
        "N" => Action::Move(NORTH, value),
        "S" => Action::Move(SOUTH, value),
        "E" => Action::Move(EAST, value),
        "W" => Action::Move(WEST, value),
        "L" => Action::Rotate((-value / 90).rem_euclid(4)),
        "R" => Action::Rotate((value / 90).rem_euclid(4)),
        "F" => Action::Forward(value),
        _ => panic!("Unexpected input {}", s),
    };
    action
}
pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut actions = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let action = parse(&line);
        actions.push(action);
    }

    // part 1
    let mut dir: i32 = 1; // Ship start facing east
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for action in actions.iter() {
        match action {
            Action::Move((dx, dy), k) => {
                x += k * dx;
                y += k * dy;
            }
            Action::Rotate(i) => dir = (dir + i).rem_euclid(4),
            Action::Forward(k) => {
                let (dx, dy) = DIRS[dir as usize];
                x += k * dx;
                y += k * dy;
            }
        }
    }

    println!("{}", x.abs() + y.abs());

    // part 2
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut wx: i32 = 1;
    let mut wy: i32 = 10;
    for action in actions.iter() {
        match action {
            Action::Move((dx, dy), k) => {
                wx += k * dx;
                wy += k * dy;
            }
            Action::Rotate(n) => {
                for _i in 0..*n {
                    let wx0 = wx;
                    let wy0 = wy;

                    wx = -wy0;
                    wy = wx0
                }
            }
            Action::Forward(k) => {
                x += k * wx;
                y += k * wy;
            }
        }
    }

    println!("{}", x.abs() + y.abs());
}
