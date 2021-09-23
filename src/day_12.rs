use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rule {
    lhs: u8,
    rhs: bool,
}

fn parse_rule(s: &str) -> Rule {
    let lhs = s[0..5].chars().fold(0, |acc, c| {
        let acc = acc << 1;
        if c == '#' {
            acc | 1
        } else {
            acc
        }
    });
    let rhs = s.chars().last().unwrap() == '#';
    Rule { lhs, rhs }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct T {
    plants: Vec<i64>,
    offset: i64,
}

impl T {
    fn get(&self, i: i64) -> bool {
        for j in self.plants.iter() {
            if *j + self.offset == i {
                return true;
            }
        }
        return false;
    }

    fn initialize(v: &[bool]) -> T {
        let mut plants: Vec<i64> = v
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b { Some(i as i64) } else { None })
            .collect();
        plants.sort();
        T { plants, offset: 0 }
    }

    fn new(v: &[i64]) -> T {
        let &offset = v.iter().min().unwrap_or(&0);
        let mut plants: Vec<_> = v.iter().map(|i| i - offset).collect();
        plants.sort();
        T { plants, offset }
    }
}

fn mask(t: &T, i: i64) -> u8 {
    let mut acc = 0;
    for di in -2..3 {
        acc = acc << 1;
        acc += t.get(i + di) as u8
    }
    acc
}

fn step(t: &T, rules: &HashMap<u8, bool>) -> T {
    let mut h = HashSet::new();
    for i in t.plants.iter() {
        for di in -2..3 {
            h.insert(i + di + t.offset);
        }
    }
    let mut next = Vec::new();
    for i in h.into_iter() {
        let m = mask(t, i);
        let &o = rules.get(&m).unwrap();
        if o {
            next.push(i)
        }
    }
    T::new(&next)
}

fn part1(initial_state: &[bool], rules: &HashMap<u8, bool>) -> i64 {
    let mut state: T = T::initialize(initial_state);
    for _ in 0..20 {
        state = step(&state, rules);
    }
    state.plants.iter().map(|i| i + state.offset).sum()
}

// For part 2, we cannot simply iterate through 50 * 10^9 steps. It turns out
// that the system that the rules describe becomes almost periodic after a while
// (to be precise: the spacing between the plants is identical from one
// generation to the other, while the position of the "first" plant changes from
// one time step to the next). We can simply compute the value of this offset at
// the step 50 * 10 ^ 9, and be done with it.
fn part2(initial_state: &[bool], rules: &HashMap<u8, bool>) -> i64 {
    let mut states: HashMap<Vec<i64>, (i64, usize)> = HashMap::new();
    let mut state = T::initialize(initial_state);
    let mut count = 0;

    let ((offset1, step1), (offset2, step2)) = loop {
        if states.contains_key(&state.plants) {
            break (states.get(&state.plants).unwrap(), (state.offset, count));
        };
        states.insert(state.plants.clone(), (state.offset, count));
        state = step(&state, rules);
        count += 1;
    };
    if step2 == step1 + 1 {
        let final_offset = (offset2 - offset1) * (50_000_000_000 as i64 - step2 as i64) + offset2;
        state.plants.iter().map(|i| i + final_offset).sum()
    } else {
        panic!()
    }
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let contents: Vec<_> = contents.split("\n\n").collect();
    let initial_state = contents[0].strip_prefix("initial state: ").unwrap();
    let initial_state: Vec<bool> = initial_state.chars().map(|c| c == '#').collect();
    let rules: HashMap<_, _> = contents[1]
        .split('\n')
        .filter_map(|r| {
            if r == "" {
                None
            } else {
                let Rule { lhs, rhs } = parse_rule(r);
                Some((lhs, rhs))
            }
        })
        .collect();
    println!("{}", part1(&initial_state, &rules));
    println!("{}", part2(&initial_state, &rules));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_rule() {
        let r = "####. => #";
        assert_eq!(
            parse_rule(r),
            Rule {
                lhs: 0b11110,
                rhs: true
            }
        );
        let r = ".###. => .";
        assert_eq!(
            parse_rule(r),
            Rule {
                lhs: 0b01110,
                rhs: false
            }
        )
    }
}
