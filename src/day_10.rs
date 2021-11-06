use crate::util::vector2::Vector2;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Star {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
}

fn parse_line(s: &str) -> Star {
    let (posx, posy, velx, vely) =
        scan_fmt!(s, "position=<{}, {}> velocity=<{}, {}>", i32, i32, i32, i32).unwrap();
    let position = Vector2::new(posx, posy);
    let velocity = Vector2::new(velx, vely);
    Star { position, velocity }
}

fn parse(s: &str) -> Vec<Star> {
    s.split('\n')
        .filter_map(|l| if l != "" { Some(parse_line(l)) } else { None })
        .collect()
}

fn step(stars: &mut Vec<Star>) {
    for star in stars.iter_mut() {
        star.position = star.position + star.velocity
    }
}

fn to_set(state: &[Star]) -> HashSet<Vector2<i32>> {
    state.iter().map(|s| s.position).collect()
}

fn proximity(state: &[Star]) -> usize {
    let state = to_set(state);
    let dirs = vec![
        Vector2::new(1, 0),
        Vector2::new(-1, 0),
        Vector2::new(0, 1),
        Vector2::new(0, -1),
    ];
    let mut count = 0;
    for &pos in state.iter() {
        let mut this_count = 0;
        for &dir in dirs.iter() {
            if state.contains(&(pos + dir)) {
                this_count += 1
            }
        }
        if 2 <= this_count {
            count += 1;
        }
    }
    count
}

// For part 1, we need to find a magic number of steps after which the stars
// display a message. We could do this step by step, but we can also infer from
// the example that this corresponds to the step at which the
fn part1(mut state: Vec<Star>) {
    let mut best_state = state.clone();
    let mut best_proximity = proximity(&best_state);
    for _i in 0..10_036 {
        step(&mut state);
        let proximity = proximity(&state);

        if proximity > best_proximity {
            best_state = state.clone();
            best_proximity = proximity;
        }
    }
    let positions: Vec<_> = best_state.iter().map(|s| s.position).collect();
    let (min, max) = crate::util::vector2::bounding_box(&positions);
    let positions: HashSet<_> = best_state.iter().map(|s| s.position).collect();
    for y in (min.y - 1)..(max.y + 2) {
        for x in (min.x - 1)..(max.x + 2) {
            let p = Vector2::new(x, y);
            if positions.contains(&p) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(&filename).unwrap();
    let stars = parse(&content);
    part1(stars)
}
