use crate::util::direction::Direction;
use crate::util::vector2::Vector2;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Cart {
    pos: Vector2<i32>,
    dir: Direction,
    id: usize,
    turns: usize,
    crashed: bool,
}

#[derive(Debug)]
struct T {
    rails: HashMap<Vector2<i32>, char>,
    carts: Vec<Cart>,
    collisions: Vec<Vector2<i32>>,
}

// -> x
// v
// y
fn parse(s: &str) -> T {
    let mut rails = HashMap::new();
    let mut carts = Vec::new();
    let mut cart_id = 0;
    let mut cart = |pos, dir| {
        let c = Cart {
            pos,
            dir,
            id: cart_id,
            turns: 0,
            crashed: false,
        };
        cart_id += 1;
        carts.push(c);
    };
    for (y, line) in s.split('\n').enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Vector2::new(x as i32, y as i32);
            match char {
                '|' | '/' | '\\' | '-' | '+' => {
                    rails.insert(pos, char);
                }
                '>' => {
                    cart(pos, Direction::East);
                    rails.insert(pos, '-');
                }
                '<' => {
                    cart(pos, Direction::West);
                    rails.insert(pos, '-');
                }
                '^' => {
                    cart(pos, Direction::North);
                    rails.insert(pos, '|');
                }
                'v' => {
                    cart(pos, Direction::South);
                    rails.insert(pos, '|');
                }
                ' ' => {}
                _ => panic!("Unexpected input: {}", char),
            }
        }
    }
    T {
        rails,
        carts,
        collisions: vec![],
    }
}

fn vel(d: Direction) -> Vector2<i32> {
    match d {
        Direction::North => Vector2::new(0, -1),
        Direction::South => Vector2::new(0, 1),
        Direction::East => Vector2::new(1, 0),
        Direction::West => Vector2::new(-1, 0),
    }
}

fn left(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
        Direction::West => Direction::South,
    }
}

fn right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

fn step(t: &T, c: Cart) -> Cart {
    let vel = vel(c.dir);
    let next = c.pos + vel;
    let dir = match (c.dir, t.rails.get(&next).unwrap()) {
        (Direction::North, '|') => Direction::North,
        (Direction::South, '|') => Direction::South,
        (Direction::North, '/') => Direction::East,
        (Direction::East, '/') => Direction::North,
        (Direction::West, '/') => Direction::South,
        (Direction::South, '/') => Direction::West,
        (Direction::North, '\\') => Direction::West,
        (Direction::East, '\\') => Direction::South,
        (Direction::West, '\\') => Direction::North,
        (Direction::South, '\\') => Direction::East,
        (Direction::East, '-') => Direction::East,
        (Direction::West, '-') => Direction::West,
        (dir, '+') => {
            if c.turns % 3 == 0 {
                left(dir)
            } else if c.turns % 3 == 1 {
                dir
            } else {
                right(dir)
            }
        }
        (dir, c) => {
            panic!("{:?},{:?}", dir, c);
        }
    };
    let turns = if *t.rails.get(&next).unwrap() == '+' {
        c.turns + 1
    } else {
        c.turns
    };
    Cart {
        pos: next,
        dir,
        turns,
        ..c
    }
}

fn next(t: &mut T) {
    t.carts.sort_by_key(|c| (c.pos.y, c.pos.x));
    for i in 0..t.carts.len() {
        if t.carts[i].crashed {
        } else {
            let next = step(t, t.carts[i]);

            if let Some((j, collision)) = t
                .carts
                .iter()
                .enumerate()
                .map(|(j, c)| (j, c.pos))
                .find(|(_j, p)| *p == next.pos)
            {
                t.carts[i] = next;
                t.carts[i].crashed = true;
                t.carts[j].crashed = true;
                t.collisions.push(collision);
            } else {
                t.carts[i] = next
            }
        }
    }
    t.carts.retain(|c| !c.crashed);
}

fn part1(t: &mut T) -> Vector2<i32> {
    while t.collisions.is_empty() {
        next(t)
    }
    t.collisions[0]
}

fn part2(t: &mut T) -> Vector2<i32> {
    while t.carts.len() > 1 {
        next(t)
    }
    t.carts[0].pos
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    println!("{:?}", part1(&mut parse(&contents)));
    println!("{:?}", part2(&mut parse(&contents)))
}
