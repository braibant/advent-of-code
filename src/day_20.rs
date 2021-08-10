use crate::vector2::Vector2;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = Vector2<i32>;

#[derive(Debug)]
struct Parsed {
    tiles: HashSet<Pos>,
    warps: Vec<(Pos, String)>,
}

struct T {
    neighbours: HashMap<Pos, Vec<Pos>>,
}

lazy_static! {
    static ref DIRS: Vec<Pos> = {
        // N, S, W, E
        vec![Vector2::new(0, -1), Vector2::new(1, 0), Vector2::new(0, 1), Vector2::new(-1, 0)]
    };
}

fn warp_pos(warps: &[(Pos, String)], label: &str) -> Vec<Pos> {
    warps
        .iter()
        .filter_map(|(p, l)| if l == label { Some(*p) } else { None })
        .collect()
}

fn warp_destination(warps: &[(Pos, String)], label: &str, src: Pos) -> Option<Pos> {
    warps.iter().find_map(|(p, l)| {
        if *p != src && l == label {
            Some(*p)
        } else {
            None
        }
    })
}

fn build(parsed: &Parsed) -> T {
    let warps_pl: HashMap<Pos, String> = parsed.warps.iter().cloned().collect();
    let mut neighbours = HashMap::new();
    let todo: Vec<_> = parsed.tiles.iter().collect();
    for &tile in parsed.tiles.iter() {
        let mut acc = vec![];
        for &d in DIRS.iter() {
            if parsed.tiles.contains(&(tile + d)) {
                acc.push(tile + d);
            }
        }
        match warps_pl.get(&tile) {
            None => {}
            Some(n) => match warp_destination(&parsed.warps, n, tile) {
                None => {}
                Some(d) => acc.push(d),
            },
        };
        neighbours.insert(tile, acc);
    }
    T { neighbours }
}

fn bfs(t: &T, src: Pos, dst: Pos) -> Option<u32> {
    let mut todo = std::collections::VecDeque::new();
    let mut visited = HashSet::new();
    todo.push_back((src, 0));
    while let Some((next, dist)) = todo.pop_front() {
        if visited.contains(&next) {
        } else {
            visited.insert(next);
            if next == dst {
                return Some(dist);
            } else {
                for n in t.neighbours.get(&next).unwrap_or(&vec![]).iter() {
                    todo.push_back((*n, dist + 1))
                }
            }
        }
    }
    return None;
}

// x ->
// v y
fn parse(content: &str) -> Parsed {
    // First, let's transform the string into a 2d matrix.
    let lines = content.split('\n');
    let mut image = vec![];
    for line in lines {
        image.push(line.chars().collect::<Vec<_>>())
    }
    let height = image.len();
    let width = image[0].len();
    let mut warps: Vec<(Pos, String)> = vec![];
    for y in 0..height {
        for x in 0..(width - 2) {
            let a = image[y][x];
            let b = image[y][x + 1];
            let c = image[y][x + 2];

            if a.is_alphabetic() && b.is_alphabetic() && c == '.' {
                warps.push((
                    Vector2::new((x + 2) as i32, y as i32),
                    format!("{}{}", a, b),
                ))
            } else if a == '.' && b.is_alphabetic() && c.is_alphabetic() {
                warps.push((Vector2::new(x as i32, y as i32), format!("{}{}", b, c)))
            }
        }
    }
    for y in 0..(height - 2) {
        for x in 0..width {
            let a = image[y][x];
            let b = image[y + 1][x];
            let c = image[y + 2][x];
            if a.is_alphabetic() && b.is_alphabetic() && c == '.' {
                warps.push((
                    Vector2::new(x as i32, (y + 2) as i32),
                    format!("{}{}", a, b),
                ))
            } else if a == '.' && b.is_alphabetic() && c.is_alphabetic() {
                warps.push((Vector2::new(x as i32, y as i32), format!("{}{}", b, c)))
            }
        }
    }
    let mut tiles = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if image[y][x] == '.' {
                tiles.insert(Vector2::new(x as i32, y as i32));
            }
        }
    }
    Parsed { tiles, warps }
}
pub fn run(filename: &str) {
    // Parsing here is a bit more complex than previous problems. Let's do this
    // in three steps: first, process the file line by line, looking for labels
    // and recording the position of the (unique) tile before or after them;
    // second, process the file column by column and do the same; then construct
    // the map.

    let contents = std::fs::read_to_string(filename).unwrap();
    let parsed = parse(&contents);
    let t = build(&parsed);
    let src = warp_pos(&parsed.warps, "AA")[0];
    let tgt = warp_pos(&parsed.warps, "ZZ")[0];
    println!("{:?}", bfs(&t, src, tgt))
}
