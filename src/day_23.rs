use crate::util::vector3::Vector3;

type Position = Vector3<i32>;

fn parse_line(s: &str) -> (Position, i32) {
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

fn parse(s: &str) -> Vec<(Position, i32)> {
    s.lines().map(|line| parse_line(line)).collect()
}

fn distance(a: Position, b: Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn part1(bots: &[(Position, i32)]) -> usize {
    let &(pos, r) = bots.iter().max_by_key(|(_, r)| r).unwrap();
    bots.iter().filter(|(p, _)| distance(*p, pos) <= r).count()
}

fn sq_distance(low: Position, high: Position, p: Position) -> i64 {
    let dx = [low.x - p.x, 0, p.x - high.x].into_iter().max().unwrap() as i64;
    let dy = [low.y - p.y, 0, p.y - high.y].into_iter().max().unwrap() as i64;
    let dz = [low.z - p.z, 0, p.z - high.z].into_iter().max().unwrap() as i64;
    dx * dx + dy * dy + dz * dz
}

// Checks if the box in range of the bot intersects the box defined by the
// corners low / high.
fn intersect(bot: Position, radius: i32, low: Position, high: Position) -> bool {}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let bots = parse(&content);
    println!("{}", part1(&bots));
    let bb = crate::util::vector3::bounding_box(bots.iter().map(|(p, _)| p));
    println!("{:?}", bb);
}
