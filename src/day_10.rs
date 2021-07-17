#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn parse(contents: &str) -> Vec<Point> {
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut acc = vec![];
    for (x, line) in lines.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                acc.push(Point {
                    x: x as i64,
                    y: y as i64,
                })
            }
        }
    }
    acc
}

// Points `a`, `b`, `c` are aligned.
fn aligned(a: &Point, b: &Point, c: &Point) -> bool {
    let cross_product = (c.y - a.y) * (b.x - a.x) - (c.x - a.x) * (b.y - a.y);
    cross_product == 0
}

// Is `c` on the ray from  `a` to `b` (i.e. all the points between `a` and `b`  and all the points `c` such that `b` is between `a` and `c`)
fn is_on_ray(a: &Point, b: &Point, c: &Point) -> bool {
    if !aligned(a, b, c) {
        return false;
    };

    let dot_produt = (c.x - a.x) * (b.x - a.x) + (c.y - a.y) * (b.y - a.y);
    0 <= dot_produt
}

// Is `c` between `a` and `b`
fn is_between(a: &Point, b: &Point, c: &Point) -> bool {
    if !aligned(a, b, c) {
        return false;
    };
    let dot_product = (c.x - a.x) * (b.x - a.x) + (c.y - a.y) * (b.y - a.y);
    let squared_length = (b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y);

    return (0 <= dot_product && dot_product <= squared_length);
}

// There is a trivial O(n^3) algorithm : For each potential location, consider all asteroids and check if their line of sight is blocked by a third-party asteroid.
// We can refine this algorithm by considering that, we are simply counting the distinct "rays" as defined by a station location and a candidate asteroid.
fn number_of_asteroids_in_sight(asteroids: &Vec<Point>, location: &Point) -> usize {
    let mut candidates = asteroids.clone();

    let mut count = 0;
    while candidates.len() != 0 {
        let candidate = candidates.pop().unwrap();
        if candidate == *location {
        } else {
            // Remove all asteroids which are between location and candidate (that we have not yet visited in this iteration) and all the asteroids which are blocked by candidate.
            candidates.retain(|c| !is_on_ray(location, &candidate, c));
            count += 1;
        }
    }
    count
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let asteroids = parse(&contents);
    println!("{}", asteroids.len());

    let max = asteroids
        .iter()
        .map(|location| number_of_asteroids_in_sight(&asteroids, location))
        .max();
    println!("{:?}", max);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let puzzle = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let asteroids = parse(puzzle);
        let max = number_of_asteroids_in_sight(&asteroids, &Point { x: 3, y: 6 });
        assert_eq!(max, 41);
    }
}
