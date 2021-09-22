use crate::util::vector2::Vector2;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

fn parse(s: &str) -> Vec<Vector2<i32>> {
    s.split('\n')
        .filter_map(|line| {
            if line == "" {
                None
            } else {
                let (x, y) = scan_fmt!(line, "{}, {}", i32, i32).unwrap();
                Some(Vector2::new(x, y))
            }
        })
        .collect()
}

// For part 1, let's call `cell(X)` the set of point that's closest to `X`. We
// can observe that, if we draw a loose bounding box around the set of points
// that we receive, if cell(X) intersects the fringe of this loose bounding box,
// it implies that cell(X) is infinite. So, we can iterate over the points in
// this loose bounding box, compute the distance to the N points in our input
// and use this to tally the size of `cell(X)` (and mark this size as infinite
// if `cell(X)` intersects the fringe of our loose bounding box.)

/// Returns ((min x, min y), (max x, max y))
fn bounding_box(points: &[Vector2<i32>]) -> (Vector2<i32>, Vector2<i32>) {
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();
    (Vector2::new(minx, miny), Vector2::new(maxx, maxy))
}

fn manhattan_distance(a: Vector2<i32>, b: Vector2<i32>) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn nearest(points: &[Vector2<i32>], p: Vector2<i32>) -> Option<Vector2<i32>> {
    let mut distances: Vec<_> = points
        .iter()
        .map(|x| (x, manhattan_distance(p, *x)))
        .collect();
    distances.sort_by_key(|(_x, d)| *d);
    match distances.len() {
        0 => panic!(),
        1 => {
            let (pa, _da) = distances[0];
            Some(*pa)
        }
        _ => {
            let (pa, da) = distances[0];
            let (_pb, db) = distances[1];
            if da == db {
                return None;
            } else {
                return Some(*pa);
            }
        }
    }
}

// ^
// |
// y
// x ->
fn part1(s: &str) -> usize {
    let points = parse(s);
    let (bl, ur) = bounding_box(&points);
    let mut areas: HashMap<Vector2<i32>, Option<usize>> = HashMap::new();
    for x in (bl.x - 1)..(ur.x + 2) {
        for y in (bl.y - 1)..(ur.y + 2) {
            let c = Vector2::new(x, y);

            match nearest(&points, c) {
                None => {}
                Some(a) => {
                    if x == bl.x - 1 || x == ur.x + 1 || y == bl.y - 1 || y == ur.y + 1 {
                        let size = areas.entry(a).or_insert(None);
                        *size = None
                    } else {
                        let size = areas.entry(a).or_insert(Some(0));
                        if size.is_none() {
                        } else {
                            let n = size.unwrap();
                            *size = Some(n + 1);
                        }
                    }
                }
            }
        }
    }
    areas.values().filter_map(|size| *size).max().unwrap()
}

// For part 2, we want to compute the size of the region that has total distance
// to the given points less than K. It's not obvious that this region is fitting
// inside the bounding box of the points that are given to us. My intuition is
// that this region is a connected space, so maybe we ccould do some flood fill
// starting from ... somewhere? Anyhow, the brute force method is also available
// and provides a result that does not depend on how tight our bounding box is,
// upon which we could gain more confidence that the function is correct by
// showing that the distance on the bounding box exceeds the threshold.
fn part2(s: &str) -> usize {
    let points = parse(s);
    let (bl, ur) = bounding_box(&points);
    let mut count = 0;
    for x in (bl.x - 1)..(ur.x + 2) {
        for y in (bl.y - 1)..(ur.y + 2) {
            let c = Vector2::new(x, y);
            let d: i32 = points.iter().map(|&p| manhattan_distance(p, c)).sum();
            if d < 10_000 {
                count += 1
            }
        }
    }
    count
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    println!("{}", part1(&content));
    println!("{}", part2(&content));
}
