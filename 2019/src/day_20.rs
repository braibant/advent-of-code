use crate::vector2::Vector2;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = Vector2<i32>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Side {
    Outer,
    Inner,
}

#[derive(Debug)]
struct Parsed {
    tiles: HashSet<Pos>,
    warps_pl: HashMap<Pos, (String, Side)>,
    warps_lp: HashMap<String, Vec<(Pos, Side)>>,
}

lazy_static! {
    static ref DIRS: Vec<Pos> = {
        // N, S, W, E
        vec![Vector2::new(0, -1), Vector2::new(1, 0), Vector2::new(0, 1), Vector2::new(-1, 0)]
    };
}

fn warp_destination(parsed: &Parsed, label: &str, src: Pos) -> Option<(Pos, Side)> {
    parsed
        .warps_lp
        .get(label)
        .unwrap_or(&vec![])
        .into_iter()
        .find_map(|(p, s)| if *p != src { Some((*p, *s)) } else { None })
}

fn neighbours(parsed: &Parsed, pos: Pos) -> Vec<Pos> {
    let mut acc = vec![];
    for &d in DIRS.iter() {
        if parsed.tiles.contains(&(pos + d)) {
            acc.push(pos + d);
        }
    }
    match parsed.warps_pl.get(&pos) {
        None => {}
        Some((n, _side)) => match warp_destination(&parsed, n, pos) {
            None => {}
            Some((d, _s)) => acc.push(d),
        },
    };
    acc
}

fn neighbours_rec(parsed: &Parsed, (pos, level): (Pos, u32)) -> Vec<(Pos, u32)> {
    let mut acc = vec![];
    for &d in DIRS.iter() {
        if parsed.tiles.contains(&(pos + d)) {
            acc.push((pos + d, level));
        }
    }
    match parsed.warps_pl.get(&pos) {
        None => {}
        Some((n, side)) => match warp_destination(&parsed, n, pos) {
            None => {}
            Some((d, dside)) => {
                if *side == dside {
                    acc.push((d, level))
                } else if *side == Side::Outer && 0 < level {
                    acc.push((d, level - 1))
                } else if *side == Side::Inner && level < 500 {
                    acc.push((d, level + 1))
                }
            }
        },
    };
    acc
}

fn bfs<T, F>(neighbours: F, src: T, dst: T) -> Option<u32>
where
    F: Fn(T) -> Vec<T>,
    T: Eq + std::hash::Hash + Copy,
{
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
                for n in neighbours(next).iter() {
                    todo.push_back((*n, dist + 1))
                }
            }
        }
    }
    None
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
    let mut warps: Vec<(Pos, String, Side)> = vec![];
    for y in 0..height {
        for x in 0..(width - 2) {
            let a = image[y][x];
            let b = image[y][x + 1];
            let c = image[y][x + 2];
            let side = if x == 0 || x == width - 3 {
                Side::Outer
            } else {
                Side::Inner
            };
            if a.is_alphabetic() && b.is_alphabetic() && c == '.' {
                warps.push((
                    Vector2::new((x + 2) as i32, y as i32),
                    format!("{}{}", a, b),
                    side,
                ))
            } else if a == '.' && b.is_alphabetic() && c.is_alphabetic() {
                warps.push((
                    Vector2::new(x as i32, y as i32),
                    format!("{}{}", b, c),
                    side,
                ))
            }
        }
    }
    for y in 0..(height - 2) {
        for x in 0..width {
            let a = image[y][x];
            let b = image[y + 1][x];
            let c = image[y + 2][x];
            let side = if y == 0 || y == height - 3 {
                Side::Outer
            } else {
                Side::Inner
            };
            if a.is_alphabetic() && b.is_alphabetic() && c == '.' {
                warps.push((
                    Vector2::new(x as i32, (y + 2) as i32),
                    format!("{}{}", a, b),
                    side,
                ))
            } else if a == '.' && b.is_alphabetic() && c.is_alphabetic() {
                warps.push((
                    Vector2::new(x as i32, y as i32),
                    format!("{}{}", b, c),
                    side,
                ))
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
    let warps_pl: HashMap<Pos, (String, Side)> = warps
        .iter()
        .cloned()
        .map(|(pos, lbl, side)| (pos, (lbl, side)))
        .collect();
    let mut warps_lp = HashMap::new();
    for (pos, label, side) in warps.iter() {
        warps_lp
            .entry(label.clone())
            .or_insert(vec![])
            .push((*pos, *side))
    }
    Parsed {
        tiles,
        warps_pl,
        warps_lp,
    }
}
pub fn run(filename: &str) {
    // Parsing here is a bit more complex than previous problems. Let's do this
    // in three steps: first, process the file line by line, looking for labels
    // and recording the position of the (unique) tile before or after them;
    // second, process the file column by column and do the same; then construct
    // the map.

    let contents = std::fs::read_to_string(filename).unwrap();
    let parsed = parse(&contents);
    // let t = build(&parsed);
    let src = parsed.warps_lp.get("AA").unwrap()[0].0;
    let tgt = parsed.warps_lp.get("ZZ").unwrap()[0].0;
    println!("{:?}", bfs(|pos| neighbours(&parsed, pos), src, tgt));
    println!(
        "{:?}",
        bfs(|pos| neighbours_rec(&parsed, pos), (src, 0), (tgt, 0))
    );
}
