use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::util::vector2::Vector2;

// -> x
// |
// V  y
type Position = Vector2<i32>;

const U: Vector2<i32> = Vector2 { x: 0, y: -1 };
const D: Vector2<i32> = Vector2 { x: 0, y: 1 };
const L: Vector2<i32> = Vector2 { x: -1, y: 0 };
const R: Vector2<i32> = Vector2 { x: 1, y: 0 };

#[derive(PartialEq, Clone, Copy, Debug)]
enum Species {
    Elf,
    Goblin,
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Unit {
    species: Species,
    last_move: i32,
    hit_points: i32,
}

const HP: i32 = 200;

impl Unit {
    fn g() -> Unit {
        Unit {
            species: Species::Goblin,
            hit_points: HP,
            last_move: -1,
        }
    }

    fn e() -> Unit {
        Unit {
            species: Species::Elf,
            hit_points: HP,
            last_move: -1,
        }
    }
}

#[derive(Debug, Clone)]
struct T {
    graph: HashMap<Position, Option<Unit>>,
    width: i32,
    height: i32,
    rounds: i32,
    score: Option<(i32, i32)>,
    elf_attack_power: i32,
}

impl T {
    fn elves(&self) -> usize {
        self.graph
            .values()
            .filter(|u| match u {
                None => false,
                Some(u) => u.species == Species::Elf,
            })
            .count()
    }

    fn print(&self) {
        let mut b = String::new();

        b.push_str("    ");
        for x in 0..=self.width {
            b.push_str(&format!("{}", x % 10));
        }

        b.push('\n');

        for y in 0..=self.height {
            let mut hps = Vec::new();
            b.push_str(&format!("{:3} ", y));
            for x in 0..=self.width {
                match self.graph.get(&Vector2::new(x, y)) {
                    None => b.push('#'),
                    Some(None) => b.push('.'),
                    Some(Some(Unit {
                        species: Species::Elf,
                        hit_points,
                        ..
                    })) => {
                        b.push('E');
                        hps.push(format!("E({})", hit_points))
                    }
                    Some(Some(Unit {
                        species: Species::Goblin,
                        hit_points,
                        ..
                    })) => {
                        b.push('G');
                        hps.push(format!("G({})", hit_points))
                    }
                }
            }
            b.push('\t');
            b.push_str(&hps.join(", "));
            b.push('\n');
        }
        println!("{}", b);
    }

    fn is_empty_tile(&self, pos: &Position) -> bool {
        matches!(self.graph.get(pos), Some(None))
    }

    fn units(&self) -> Vec<Unit> {
        self.graph.values().filter_map(|&x| x).collect()
    }

    fn targets(&self, from: Species) -> Vec<Position> {
        self.graph
            .iter()
            .filter_map(|(&p, v)| match v {
                None => None,
                Some(u) => {
                    if u.species != from {
                        Some(p)
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    // If there is no path to any targets, returns 0, otherwise returns a the first direction on a path.
    fn find_path_to_closest_target(&self, pos: Position, targets: &[Position]) -> Vector2<i32> {
        let dirs = vec![U, L, R, D];
        let targets: HashSet<_> = targets.iter().copied().collect();
        let in_reach: HashSet<_> = targets
            .iter()
            .flat_map(|&p| vec![p + U, p + D, p + L, p + R])
            .collect();
        let mut todo = Vec::new();
        let mut visited = HashSet::new();

        for &d in dirs.iter() {
            todo.push((pos + d, d));
        }

        while !todo.is_empty() {
            // First, check if we have reached an empty tile in a reach of a target
            let mut ir: Vec<_> = todo
                .iter()
                .filter(|(p, _)| self.is_empty_tile(p) && in_reach.contains(p))
                .collect();
            if !ir.is_empty() {
                // Find the best target.
                ir.sort_by_key(|(p, _)| (p.y, p.x));
                let &(p, _di) = ir[0];
                let valid_dirs: HashSet<_> = ir
                    .iter()
                    .filter_map(|&&(q, d)| if p == q { Some(d) } else { None })
                    .collect();
                let order = vec![U, L, R, D];
                for d in order.iter() {
                    if valid_dirs.contains(d) {
                        return *d;
                    }
                }
            } else {
                let mut next = Vec::new();
                for (p, di) in todo.iter().cloned() {
                    if !visited.contains(&p) && self.is_empty_tile(&p) {
                        for &d in dirs.iter() {
                            next.push((p + d, di))
                        }
                        visited.insert(p);
                    };
                }
                todo.clear();
                todo.append(&mut next);
            }
        }
        Vector2::new(0, 0)
    }

    fn attack(&mut self, pos: Position, unit: Unit) {
        let order = vec![pos + U, pos + L, pos + R, pos + D];
        let mut other_units: Vec<_> = order
            .into_iter()
            .filter_map(|p| match self.graph.get(&p) {
                None => None,
                Some(None) => None,
                Some(Some(other)) => {
                    if other.species != unit.species {
                        Some((p, *other))
                    } else {
                        None
                    }
                }
            })
            .collect();

        other_units.sort_by_key(|(p, u)| (u.hit_points, p.y, p.x));

        let (other_pos, mut other) = other_units[0];

        let attack_power = match unit.species {
            Species::Elf => self.elf_attack_power,
            Species::Goblin => 3,
        };

        if other.hit_points > attack_power {
            other.hit_points -= attack_power;
            self.graph.insert(other_pos, Some(other));
        } else {
            self.graph.insert(other_pos, None);
        }
    }

    fn step(&mut self) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let p1 = Vector2::new(x, y);
                // Get a copy of the unit to process, if any.
                let unit_to_process = match self.graph.get_mut(&p1) {
                    None => None,
                    Some(None) => None,
                    Some(Some(u1)) => {
                        if u1.last_move < self.rounds {
                            u1.last_move = self.rounds;
                            Some(*u1)
                        } else {
                            None
                        }
                    }
                };
                match unit_to_process {
                    None => {}
                    Some(u1) => {
                        let targets = self.targets(u1.species);
                        if targets.is_empty() {
                            // If we cannot a find a target, we compute the score and break
                            let hit_points: i32 = self.units().iter().map(|u| u.hit_points).sum();
                            self.score = Some((self.rounds, hit_points));
                            return;
                        }
                        let in_range_of_targets: HashSet<_> = targets
                            .iter()
                            .flat_map(|&p| vec![p + U, p + L, p + R, p + D])
                            .collect();

                        if in_range_of_targets.is_empty() {
                            // Finish the turn
                        } else if in_range_of_targets.contains(&p1) {
                            self.attack(p1, u1)
                        } else {
                            // Find path toward closest square (breaking ties by chosing the first square in reading order) and move in that direction, then attack, if possible.
                            let dir = self.find_path_to_closest_target(p1, &targets);
                            self.graph.insert(p1, None);
                            assert!(self.is_empty_tile(&(p1 + dir)), "{:?} {:?}", p1, dir);
                            self.graph.insert(p1 + dir, Some(u1));
                            if in_range_of_targets.contains(&(p1 + dir)) {
                                self.attack(p1 + dir, u1)
                            }
                        }
                    }
                }
            }
        }
        self.rounds += 1;
    }
}

fn parse(s: &str) -> T {
    let s = s.replace(" ", "");
    let lines = s.split('\n');
    let mut graph = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in lines.enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let pos = Vector2::new(x, y);
            match c {
                '.' => {
                    graph.insert(pos, None);
                }
                'G' => {
                    graph.insert(pos, Some(Unit::g()));
                }
                'E' => {
                    graph.insert(pos, Some(Unit::e()));
                }
                '#' => {}
                _ => panic!("Unexpected puzzle input {}", c),
            };
            width = std::cmp::max(width, x);
        }
        height = std::cmp::max(height, y);
    }
    T {
        rounds: 0,
        graph,
        height,
        width,
        score: None,
        elf_attack_power: 3,
    }
}

fn part1(mut t: T, debug: bool) -> (i32, i32) {
    let score = loop {
        match t.score {
            None => {
                if debug {
                    println!("Round {}", t.rounds);
                    t.print();
                    println!();
                }
                t.step()
            }
            Some(score) => break score,
        }
    };
    t.print();
    score
}

fn part2(t0: T) -> (i32, i32, i32) {
    let elves = t0.elves();
    let mut ap = 4;
    loop {
        let mut t = t0.clone();
        t.elf_attack_power = ap;
        while t.elves() == elves && t.score.is_none() {
            t.step()
        }
        if t.score.is_some() {
            let (rounds, hp) = t.score.unwrap();
            return (ap, rounds, hp);
        } else {
            ap += 1;
        }
    }
}

pub fn run(s: &str) {
    let contents = std::fs::read_to_string(s).unwrap();
    let t = parse(&contents);
    let (rounds, hps) = part1(t.clone(), false);
    println!(
        "rounds: {}, hitpoints: {}, score: {}",
        rounds,
        hps,
        rounds * hps
    );
    let (ap, rounds, hps) = part2(t);
    println!(
        "ap: {}, rounds: {}, hitpoints: {}, score: {}",
        ap,
        rounds,
        hps,
        rounds * hps
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expect_test(s: &str) -> T {
        // Let's cut the first part, i.e., the puzzle, and drop the remainder of
        // the line once we reach a space.
        let parts: Vec<_> = s.split("\n\n").collect();
        let puzzle = parts[0];
        let puzzle = puzzle
            .lines()
            .map(|l| l.split_ascii_whitespace().next().unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        parse(&puzzle)
    }

    const E1: &str = "#######       #######
#G..#E#       #...#E#   E(200)
#E#E.E#       #E#...#   E(197)
#G.##.#  -->  #.E##.#   E(185)
#...#E#       #E..#E#   E(200), E(200)
#...E.#       #.....#
#######       #######

Combat ends after 37 full rounds
Elves win with 982 total hit points left
Outcome: 37 * 982 = 36334";

    #[test]
    fn test_e1() {
        let t = parse_expect_test(E1);
        assert_eq!(part1(t, true), (37, 982));
    }

    const E2: &str = "#######       #######   
#E..EG#       #.E.E.#   E(164), E(197)
#.#G.E#       #.#E..#   E(200)
#E.##E#  -->  #E.##.#   E(98)
#G..#.#       #.E.#.#   E(200)
#..E#.#       #...#.#   
#######       #######   

Combat ends after 46 full rounds
Elves win with 859 total hit points left
Outcome: 46 * 859 = 39514";

    #[test]
    fn test_e2() {
        let t = parse_expect_test(E2);
        assert_eq!(part1(t, false), (46, 859));
    }

    const E3: &str = "#######       #######   
#E.G#.#       #G.G#.#   G(200), G(98)
#.#G..#       #.#G..#   G(200)
#G.#.G#  -->  #..#..#   
#G..#.#       #...#G#   G(95)
#...E.#       #...G.#   G(200)
#######       #######   

Combat ends after 35 full rounds
Goblins win with 793 total hit points left
Outcome: 35 * 793 = 27755";

    #[test]
    fn test_e3() {
        let t = parse_expect_test(E3);
        assert_eq!(part1(t, false), (35, 793));
    }
    const E4: &str = "#######       #######   
#.E...#       #.....#   
#.#..G#       #.#G..#   G(200)
#.###.#  -->  #.###.#   
#E#G#G#       #.#.#.#   
#...#G#       #G.G#G#   G(98), G(38), G(200)
#######       #######   

Combat ends after 54 full rounds
Goblins win with 536 total hit points left
Outcome: 54 * 536 = 28944";

    #[test]
    fn test_e4() {
        let t = parse_expect_test(E4);
        assert_eq!(part1(t, false), (54, 536));
    }

    const E5: &str = "#########       #########   
#G......#       #.G.....#   G(137)
#.E.#...#       #G.G#...#   G(200), G(200)
#..##..G#       #.G##...#   G(200)
#...##..#  -->  #...##..#   
#...#...#       #.G.#...#   G(200)
#.G...G.#       #.......#   
#.....G.#       #.......#   
#########       #########   

Combat ends after 20 full rounds
Goblins win with 937 total hit points left
Outcome: 20 * 937 = 18740";

    #[test]
    fn test_e5() {
        let t = parse_expect_test(E5);
        assert_eq!(part1(t, false), (20, 937));
    }

    #[test]
    fn test_parse() {
        let s = "#####
                      #.G.#
                      #...#   
                      #####";
        let t = parse(s);
        assert_eq!(
            t.graph.get(&Vector2::new(2, 1)).copied().unwrap(),
            Some(Unit::g())
        );
    }
    #[test]
    fn test_example_1() {
        let s = "#######
                      #.G...#
                      #...EG#
                      #.#.#G#
                      #..G#E#
                      #.....#   
                      #######";
        let t = parse(s);
        assert_eq!(part1(t, false), (47, 590));
    }

    const E6: &str = "####### 
#E..G.# 
#...#.# 
#.G.#G# 
#######
";

    #[test]
    fn test_target_ordering_1() {
        let t = parse(E6);
        let targets = t.targets(Species::Elf);
        let d = t.find_path_to_closest_target(Vector2::new(1, 1), &targets);
        assert_eq!(d, R);
        let d = t.find_path_to_closest_target(Vector2::new(1, 2), &targets);
        assert_eq!(d, R);
    }

    const E7: &str = "#######
#.E...#
#.....#
#...G.#
#######";

    #[test]
    fn test_target_ordering_2() {
        let t = parse(E7);
        let targets = t.targets(Species::Elf);
        let d = t.find_path_to_closest_target(Vector2::new(2, 1), &targets);
        assert_eq!(d, R);
    }
}
