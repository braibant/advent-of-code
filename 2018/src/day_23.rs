use std::collections::BinaryHeap;

use crate::util::vector3::Vector3;

type Position = Vector3<i64>;

fn parse_line(s: &str) -> (Position, i64) {
    let parts: Vec<_> = s.split(", ").collect();
    let pos: Vec<_> = parts[0]
        .strip_prefix("pos=<")
        .unwrap()
        .strip_suffix(">")
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let radius = parts[1].strip_prefix("r=").unwrap().parse().unwrap();
    (Vector3::new(pos[0], pos[1], pos[2]), radius)
}

fn parse(s: &str) -> Vec<(Position, i64)> {
    s.lines().map(|line| parse_line(line)).collect()
}

fn manhattan_distance(a: Position, b: Position) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn part1(bots: &[(Position, i64)]) -> usize {
    let &(pos, r) = bots.iter().max_by_key(|(_, r)| r).unwrap();
    bots.iter()
        .filter(|(p, _)| manhattan_distance(*p, pos) <= r)
        .count()
}

fn bots_in_range(bots: &[(Position, i64)], pos: Position) -> usize {
    bots.iter()
        .filter(|&&(p, r)| manhattan_distance(p, pos) <= r)
        .count()
}

// Distance of a point to a cube
fn distance_to_box(low: Position, high: Position, p: Position) -> i64 {
    let dx = [low.x - p.x, 0, p.x - high.x].into_iter().max().unwrap();
    let dy = [low.y - p.y, 0, p.y - high.y].into_iter().max().unwrap();
    let dz = [low.z - p.z, 0, p.z - high.z].into_iter().max().unwrap();
    dx + dy + dz
}

// Checks if the box in range of the bot intersects the box defined by the
// corners low / high.
fn bot_in_range_of_box(bot: Position, radius: i64, low: Position, high: Position) -> bool {
    distance_to_box(low, high, bot) <= radius as i64
}

fn bots_in_range_of_box(bots: &[(Position, i64)], low: Position, high: Position) -> usize {
    bots.iter()
        .filter(|&&(p, r)| bot_in_range_of_box(p, r, low, high))
        .count()
}

fn small_box_size(low: Position, high: Position) -> bool {
    high.x - low.x < 5 && high.y - low.y < 5 && high.z - low.z < 5
}

fn box_size(low: Position, high: Position) -> i64 {
    (high.x - low.x + 1)
        .saturating_mul(high.y - low.y + 1)
        .saturating_mul(high.z - low.z + 1)
}

fn part2(bots: &[(Position, i64)]) -> i64 {
    let origin = Vector3::new(0, 0, 0);
    let mut heap = BinaryHeap::new();
    let (low, high) = crate::util::vector3::bounding_box(bots.iter().map(|(p, _)| p)).unwrap();

    heap.push((
        bots_in_range_of_box(bots, low, high),
        // box_size(low, high),
        low,
        high,
    ));
    let mut best_cover = 0;
    let mut best_distance = i64::MAX;
    while let Some((n, low, high)) = heap.pop() {
        if n < best_cover {
            break;
        }
        if n == best_cover && manhattan_distance(origin, low) > best_distance {
            // Do nothing
        } else if small_box_size(low, high) {
            // iter through the box
            for x in low.x..=high.x {
                for y in low.y..=high.y {
                    for z in low.z..=high.z {
                        let pos = Vector3::new(x, y, z);
                        let cover = bots_in_range(bots, pos);
                        let dist = manhattan_distance(origin, pos);
                        if cover > best_cover || (cover == best_cover && dist < best_distance) {
                            best_cover = cover;
                            best_distance = dist
                        }
                    }
                }
            }
        } else {
            // println!(
            //     "[{:4}] n:{:4} best:{:4} size:{:7} bd:{:5} d:{:5} {:?} {:?}",
            //     heap.len(),
            //     n,
            //     best_cover,
            //     box_size(low, high),
            //     best_distance,
            //     manhattan_distance(origin, low),
            //     low,
            //     high
            // );

            // subdivide the box
            let midx = low.x + (high.x - low.x) / 2;
            let midy = low.y + (high.y - low.y) / 2;
            let midz = low.z + (high.z - low.z) / 2;
            let x = vec![low.x, midx, high.x];
            let y = vec![low.y, midy, high.y];
            let z = vec![low.z, midz, high.z];
            for p in [
                (0, 0, 0),
                (0, 0, 1),
                (0, 1, 0),
                (0, 1, 1),
                (1, 0, 0),
                (1, 0, 1),
                (1, 1, 0),
                (1, 1, 1),
            ] {
                // There is a bit of overlap here.
                let low = Vector3::new(x[p.0], y[p.1], z[p.2]);
                let high = Vector3::new(x[p.0 + 1], y[p.1 + 1], z[p.2 + 1]);
                let n = bots_in_range_of_box(bots, low, high);

                heap.push((n, low, high));
            }
        }
    }

    best_distance
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let bots = parse(&content);
    println!("{}", part1(&bots));
    println!("{}", part2(&bots));
}
