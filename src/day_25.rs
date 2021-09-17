use crate::direction::Direction;
use crate::graph;
use crate::intcode;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
struct Room {
    name: String,
    directions: Vec<Direction>,
    items: Vec<String>,
}

fn parse(s: &str) -> Option<Room> {
    let mut name = None;
    let mut directions = Vec::new();
    let mut items = Vec::new();
    for line in s.lines() {
        if line.starts_with("==") {
            //
            if name.is_some() {
                directions.clear();
                items.clear();
            };
            name = Some(
                line.strip_prefix("== ")
                    .unwrap()
                    .strip_suffix(" ==")
                    .unwrap()
                    .to_string(),
            );
        } else if line == "- north" {
            directions.push(Direction::North);
        } else if line == "- south" {
            directions.push(Direction::South);
        } else if line == "- east" {
            directions.push(Direction::East);
        } else if line == "- west" {
            directions.push(Direction::West);
        } else if line.starts_with("- ") {
            items.push(line.strip_prefix("- ").unwrap().to_string());
        }
    }
    let name = name?;
    Some(Room {
        name,
        directions,
        items,
    })
}

fn room(vm: &mut intcode::T) -> Result<Room, String> {
    let buf = vm.get_string();
    println!("{}", buf);
    match parse(&buf) {
        None => Err(buf),
        Some(room) => Ok(room),
    }
}

fn push_direction(vm: &mut intcode::T, dir: Direction) {
    match dir {
        Direction::North => vm.push_str("north\n"),
        Direction::South => vm.push_str("south\n"),
        Direction::East => vm.push_str("east\n"),
        Direction::West => vm.push_str("west\n"),
    }
}

fn interactive_input(vm: &mut intcode::T) -> String {
    let stdin = io::stdin();
    // interactive mode
    let input = stdin.lock().lines().next();
    input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string()
}

struct T {
    rooms: HashMap<String, Room>,
    edges: HashMap<(String, Direction), String>,
    current: Room,
}

impl graph::Neighbours<String, Direction> for T {
    fn neighbours<Q>(&self, node: &Q) -> Vec<(String, Direction)>
    where
        String: std::borrow::Borrow<Q>,
        Q: ToOwned<Owned = String> + Clone,
    {
        let mut result = vec![];
        for dir in Direction::each() {
            match self.edges.get(&(node.to_owned(), dir)) {
                None => {}
                Some(next) => result.push((next.clone(), dir)),
            }
        }
        result
    }
}

impl T {
    fn new(current: Room) -> T {
        T {
            rooms: vec![(current.name.clone(), current.clone())]
                .into_iter()
                .collect(),
            current,
            edges: HashMap::new(),
        }
    }

    fn visit(&mut self, dir: Direction, room: &Room) {
        let name = room.name.clone();
        if self.rooms.contains_key(&name) {
        } else {
            self.rooms.insert(name.clone(), room.clone());
            self.edges
                .insert((self.current.name.clone(), dir), name.clone());
            self.edges
                .insert((name.clone(), dir.opposite()), self.current.name.clone());
        };
        self.current = room.clone();
    }

    fn print_items(&self) {
        for room in self.rooms.values() {
            for item in room.items.iter() {
                println!("{}", item)
            }
        }
    }

    fn find(&self, target: &String) -> Vec<(Direction)> {
        let path = graph::bfs(self, &self.current.name, target);
        path.unwrap_or(vec![])
    }
}

fn explore(program: &[i64]) -> T {
    let mut vm = intcode::T::new(&program);
    let mut state = T::new(room(&mut vm).unwrap());
    let mut to_visit: Vec<(Direction)> = state.current.directions.clone();
    while !to_visit.is_empty() {
        let dir = to_visit.pop().unwrap();
        push_direction(&mut vm, dir);
        match room(&mut vm) {
            Err(buf) => {
                println!("{:?}", buf)
            }
            Ok(r) => {
                println!("{:?} -> {:?}", dir, r);
                if r.name == state.current.name {
                    println!("ERROR: Could not follow dir {:?}", dir)
                } else {
                    if state.rooms.contains_key(&r.name) {
                    } else {
                        to_visit.push(dir.opposite());
                        to_visit.extend(r.directions.iter().filter(|&d| d.opposite() != dir));
                    }
                    state.visit(dir, &r)
                }
            }
        }
    }
    state
}

fn goto(state: &mut T, vm: &mut intcode::T, room_name: &str) {
    let directions = state.find(&room_name.to_string());
    for &dir in directions.iter() {
        push_direction(vm, dir);
        if let Ok(r) = room(vm) {
            state.current = r
        };
    }
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, x)| x.clone())
                .collect()
        })
        .collect()
}

pub fn run(filename: &str) {
    let program = intcode::read_intcode_program(filename);

    let mut state = explore(&program);
    let mut vm = intcode::T::new(&program);
    let items = vec![
        ("Navigation", "easter egg"),
        ("Warp Drive Maintenance", "mug"),
        ("Storage", "prime number"),
        ("Stables", "mouse"),
        ("Corridor", "astronaut ice cream"),
        ("Sick Bay", "ornament"),
        ("Gift Wrapping Center", "wreath"),
        ("Hot Chocolate Fountain", "hypercube"),
    ];

    for (room_name, item) in items.iter().cloned() {
        goto(&mut state, &mut vm, &room_name.to_string());
        vm.push_str(&format!("take {}\n", item));
        let buf = vm.get_string();
        println!("{}", buf)
    }
    goto(&mut state, &mut vm, &"Security Checkpoint");
    for (_, item) in items.iter() {
        vm.push_str(&format!("drop {}\n", item));
        let buf = vm.get_string();
        println!("{}", buf)
    }
    let items: Vec<&str> = items.into_iter().map(|(_, item)| item).collect();
    let combinations: Vec<Vec<&str>> = powerset(&items);
    for items in combinations.iter() {
        for item in items.iter() {
            println!("$ take {}\n", item);
            vm.push_str(&format!("take {}\n", item));
            let buf = vm.get_string();
            println!("{}", buf)
        }
        push_direction(&mut vm, Direction::North);
        let buf = vm.get_string();
        if !(buf.contains("heavier") || buf.contains("lighter")) {
            println!("{}", buf);
            break;
        } else {
            for item in items.iter() {
                println!("$ drop {}\n", item);
                vm.push_str(&format!("drop {}\n", item));
                let buf = vm.get_string();
                println!("{}", buf)
            }
        }
    }
    loop {
        println!("$");
        let s = interactive_input(&mut vm);
        if s == "rooms" {
            for room in state.rooms.values() {
                println!("{:?}", room)
            }
        } else if s == "current" {
            println!("{:?}", state.current);
        } else if s == "items" {
            state.print_items()
        } else if s.starts_with("goto ") {
            let target = s.strip_prefix("goto ").unwrap();
            goto(&mut state, &mut vm, &target.to_string())
        } else {
            vm.push_str(&s);
            vm.push_u8(b'\n');
            match room(&mut vm) {
                Ok(r) => {
                    state.current = r;
                }
                Err(_) => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing_1() {
        let s = "== Hull Breach ==
You got in through a hole in the floor here. To keep your ship from also freezing, the hole has been sealed.

Doors here lead:
- north
- west
";
        assert_eq!(
            parse(s).unwrap(),
            Room {
                name: "Hull Breach".to_string(),
                directions: vec![Direction::North, Direction::West],
                items: vec![]
            }
        )
    }
}
