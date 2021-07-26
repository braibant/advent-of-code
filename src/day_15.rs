// We need to do the following.
// 1) Data structure that represents the state of the world, which supports
//    querying for the list of points which have not yet been visited, querying
//    for a path between two points, and finally, will allow us to compute the
//    shortest path between the start point and the end.
// 2) Explore the world. We assume that the reachable set of points is finite &
//    "small". We could decide to change the exploration strategy once we have
//    found the oxygen generator, but this feels more complicated.
use crate::intcode;
use crate::vector2::Vector2;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Dir {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}

impl Dir {
    fn to_vector2(&self) -> Vector2 {
        match self {
            Dir::N => Vector2::new(0, 1),
            Dir::S => Vector2::new(0, -1),
            Dir::W => Vector2::new(-1, 0),
            Dir::E => Vector2::new(1, 0),
        }
    }
}

lazy_static! {
    static ref DIRS: Vec<(Dir, Vector2)> = {
        // N, S, W, E
        let dirs = vec![(Dir::N, Dir::N.to_vector2()),
         (Dir::S, Dir::S.to_vector2()), (Dir::W, Dir::W.to_vector2()),(Dir::E, Dir::E.to_vector2())];
        dirs
    };
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Wall,
    Tile,
}

#[derive(Debug)]
struct T {
    world: HashMap<Vector2, Cell>,
    to_visit: Vec<Vector2>,
    pos: Vector2,
    oxygen: Option<Vector2>,
    instructions: Vec<Dir>,
}

impl T {
    fn new() -> T {
        T {
            world: HashMap::new(),
            to_visit: DIRS.iter().map(|(_, d)| d.clone()).collect(),
            pos: Vector2::new(0, 0),
            oxygen: None,
            instructions: vec![],
        }
    }

    // It might be the case that cells are present in to_visit multiple times.
    fn move_to(&mut self, pos: Vector2) {
        if self.world.insert(pos, Cell::Tile).is_none() {
            for (_, dir) in DIRS.iter() {
                let next = pos + *dir;
                if !self.world.contains_key(&next) {
                    self.to_visit.push(next)
                }
            }
        };
        self.pos = pos
    }

    fn wall(&mut self, pos: Vector2) {
        self.world.insert(pos, Cell::Wall);
        self.instructions.clear()
    }

    // Returns the path to the next cell to visit
    fn next(&mut self) -> Option<Dir> {
        // Filter out the elements that we have visited in the meantime if any
        // (this is not needed, with the current exploration strategy)
        let world = &self.world;
        self.to_visit.retain(|p| !world.contains_key(p));
        if self.instructions.is_empty() {
            let tgt = self.to_visit.iter().min_by_key(|p| p.norm1(&self.pos))?;
            self.instructions = self.shortest_path(&self.pos, tgt).unwrap()
        };
        self.instructions.pop()
    }

    fn shortest_path(&self, src: &Vector2, tgt: &Vector2) -> Option<Vec<Dir>> {
        let mut visited = HashSet::new();
        let mut parent: HashMap<Vector2, (Vector2, Dir)> = HashMap::new();
        let mut q: VecDeque<Vector2> = VecDeque::new();
        q.push_back(*src);
        while !q.is_empty() {
            let v: Vector2 = q.pop_front().unwrap();
            if v == *tgt {
                let mut ptr = v;
                let mut path = vec![];
                while ptr != *src {
                    let (prev, dir) = parent.get(&ptr).unwrap();
                    path.push(*dir);
                    ptr = *prev;
                }
                return Some(path);
            };
            for (dir, dirv) in DIRS.iter() {
                match self.world.get(&(v + *dirv)) {
                    None | Some(Cell::Tile) => {
                        if visited.insert(v + *dirv) {
                            q.push_back(v + *dirv);
                            parent.insert(v + *dirv, (v, *dir));
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn exploration_complete(&self) -> bool {
        self.to_visit.is_empty()
    }
}

fn flood_fill(t: &T) -> i64 {
    let mut todo = VecDeque::new();
    let mut visited = HashSet::new();
    let mut time = 0;
    todo.push_back((t.oxygen.unwrap(), 0));
    while !todo.is_empty() {
        let (e, d) = todo.pop_front().unwrap();
        if !visited.contains(&e) && *t.world.get(&e).unwrap() == Cell::Tile {
            visited.insert(e);
            for (_, dir) in DIRS.iter() {
                todo.push_back((e + *dir, d + 1))
            }
            if time < d {
                time = d
            }
        }
    }
    time
}

// 0: The repair droid hit a wall. Its position has not changed.
// 1: The repair droid has moved one step in the requested direction.
// 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(filename);
    let mut vm = intcode::T::new(&program);
    let mut state = T::new();
    while !vm.is_halted() && !state.exploration_complete() {
        if let Some(dir) = state.next() {
            vm.push(dir as i64);
            let out = vm.pop().unwrap();
            match out {
                0 => state.wall(state.pos + dir.to_vector2()),
                1 => state.move_to(state.pos + dir.to_vector2()),
                2 => {
                    let next = state.pos + dir.to_vector2();
                    state.oxygen = Some(next);
                    state.move_to(next)
                }
                _ => panic!(),
            }
        }
    }

    let minx = state.world.keys().map(|v| v.x).min().unwrap();
    let maxx = state.world.keys().map(|v| v.x).max().unwrap();
    let miny = state.world.keys().map(|v| v.y).min().unwrap();
    let maxy = state.world.keys().map(|v| v.y).max().unwrap();

    let oxygen = state.oxygen.unwrap();
    for y in miny..(maxy + 1) {
        print!("{:>4} ", y);
        for x in minx..(maxx + 1) {
            let p = Vector2::new(x, y);
            if p == oxygen {
                print!("O")
            } else if p == Vector2::new(0, 0) {
                print!("S")
            } else {
                match state.world.get(&p) {
                    None => print!("+"),
                    Some(Cell::Wall) => print!("#"),
                    Some(Cell::Tile) => print!("."),
                }
            }
        }
        println!("")
    }

    let path: Vec<_> = state
        .shortest_path(&Vector2::new(0, 0), &state.oxygen.unwrap())
        .unwrap();

    // path.reverse();
    // let mut pos = Vector2::new(0, 0);
    // for dir in path.iter() {
    //     pos = pos + dir.to_vector2();
    //     println!("{:?}", pos);
    // }
    println!("{:?}", path.len());
    println!("{}", flood_fill(&state));
}
