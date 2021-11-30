use crate::util::direction::Direction;
use crate::util::vector2::Vector2;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Position = Vector2<i32>;

#[derive(Debug, Clone)]
enum T {
    Dir(Direction),
    Sequence(Vec<T>),
    Disjunction(Vec<T>),
}

impl T {
    fn sequence(mut v: Vec<T>) -> T {
        if v.len() == 1 {
            v.remove(0)
        } else {
            T::Sequence(v)
        }
    }

    fn disjunction(mut v: Vec<T>) -> T {
        if v.len() == 1 {
            v.remove(0)
        } else {
            T::Disjunction(v)
        }
    }
}

use nom::multi::separated_list0;
use nom::Finish;
use nom::IResult;

fn parse_dir(input: &str) -> IResult<&str, T> {
    use nom::branch::alt;
    use nom::character::complete::char;
    use nom::combinator::map;
    let n = map(char('N'), |_| Direction::North);
    let s = map(char('S'), |_| Direction::South);
    let e = map(char('E'), |_| Direction::East);
    let w = map(char('W'), |_| Direction::West);
    let parser = alt((n, s, e, w));
    map(parser, |d| T::Dir(d))(input)
}

fn parse_disjunction(input: &str) -> IResult<&str, T> {
    use nom::character::complete::char;
    use nom::combinator::map;
    use nom::sequence::delimited;
    let p = delimited(char('('), separated_list0(char('|'), parse_t), char(')'));
    map(p, |v| T::disjunction(v))(input)
}

fn parse_t(input: &str) -> IResult<&str, T> {
    use nom::branch::alt;
    use nom::combinator::map;
    use nom::multi::many0;
    let p = many0(alt((parse_dir, parse_disjunction)));
    map(p, |v| T::sequence(v))(input)
}

fn parse(input: &str) -> T {
    let (_, result) = nom::combinator::all_consuming(parse_t)(input)
        .finish()
        .unwrap();
    result
}
fn step(d: Direction) -> Vector2<i32> {
    match d {
        Direction::East => Vector2::new(1, 0),
        Direction::West => Vector2::new(-1, 0),
        Direction::North => Vector2::new(0, 1),
        Direction::South => Vector2::new(0, -1),
    }
}
struct State {
    rooms: HashMap<Position, i32>,
    graph: HashMap<Position, HashSet<Position>>,
}

#[derive(Clone)]
struct Positions(HashSet<Position>);

impl Positions {
    fn new() -> Self {
        Positions(HashSet::new())
    }
    fn insert(&mut self, p: Position) {
        self.0.insert(p);
    }
}

impl State {
    fn new() -> State {
        State {
            rooms: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    fn step(&mut self, pos: &Positions, d: Direction) -> Positions {
        let mut next = Positions::new();
        for &p in pos.0.iter() {
            let neighbors = self.graph.entry(p).or_insert(HashSet::new());
            let d = step(d);
            neighbors.insert(p + d);
            next.insert(p + d)
        }
        next
    }

    fn visit(&mut self, s: &str) {
        let mut pos = Positions::new();
        pos.insert(Vector2::new(0, 0));
        let mut lpar: Vec<Positions> = Vec::new();
        let mut rpar = Vec::new();

        for (i, c) in s.chars().enumerate() {
            println!("{}", i);

            match c {
                'N' => pos = self.step(&pos, Direction::North),
                'S' => pos = self.step(&pos, Direction::South),
                'E' => pos = self.step(&pos, Direction::East),
                'W' => pos = self.step(&pos, Direction::West),
                '(' => {
                    lpar.push(pos.clone());
                    rpar.push(Positions::new())
                }
                '|' => {
                    let mut cur = rpar.pop().unwrap();
                    for &p in pos.0.iter() {
                        cur.insert(p)
                    }
                    rpar.push(cur);
                    pos = lpar.last().unwrap().clone();
                }
                ')' => {
                    pos = rpar.pop().unwrap();
                    lpar.pop();
                }
                _ => panic!("Unexpected char {}", c),
            }
        }
    }

    fn distances(&self) -> HashMap<Position, i32> {
        let mut visited = HashMap::new();
        let mut todo = VecDeque::new();
        todo.push_back((Vector2::new(0, 0), 0 as i32));
        while let Some((p, d)) = todo.pop_front() {
            if !visited.contains_key(&p) {
                visited.insert(p, d);
                match self.graph.get(&p) {
                    None => {}
                    Some(hs) => {
                        for &n in hs.iter() {
                            todo.push_back((n, d + 1))
                        }
                    }
                }
            }
        }
        visited
    }
}

fn distances(t: &str) -> HashMap<Position, i32> {
    let mut state = State::new();
    // let mut acc = vec![];
    // state.visit1(&t, Vector2::new(0, 0), 0, &mut acc);
    state.visit(t);
    state.distances()
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    // let t = parse(&content[1..content.len() - 2]);
    let distances = distances(&content[1..content.len() - 2]);
    let &part1 = distances.values().max().unwrap();
    let part2 = distances.values().filter(|&&d| d >= 1000).count();
    println!("{}", part1);
    println!("{}", part2);
}

#[cfg(test)]
mod test {
    use super::*;
    const E0: &str = "WNE";
    const E1: &str = "ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))";
    const E2: &str = "WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))";

    fn part1(distances: &HashMap<Vector2<i32>, i32>) -> i32 {
        *distances.values().max().unwrap()
    }

    #[test]
    fn test_e0() {
        let distances = distances(E0);

        assert_eq!(part1(&distances), 3)
    }
    #[test]
    fn test_e1() {
        let distances = distances(E1);
        assert_eq!(part1(&distances), 23)
    }

    #[test]
    fn test_e2() {
        let distances = distances(E2);
        assert_eq!(part1(&distances), 31)
    }
}
