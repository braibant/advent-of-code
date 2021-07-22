#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn parse(contents: &str) -> (Vec<Point>, Option<Point>) {
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut acc = vec![];
    let mut center = None;
    let mut x = 0;
    let mut y = 0;
    for line in lines.iter() {
        x = 0;
        for char in line.chars() {
            match char {
                '#' => {
                    acc.push(Point {
                        x: x as i64,
                        y: y as i64,
                    });
                    x += 1;
                }
                'X' => {
                    center = Some(Point {
                        x: x as i64,
                        y: y as i64,
                    });
                    x += 1;
                }
                '.' => x += 1,
                ' ' => {}
                _ => panic!("unexpected {}", char),
            }
        }
        y += 1;
    }
    (acc, center)
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

/// Computes the sign of the acute angle between P1 P2 and P1 P3.
// If the result is positive (resp. negative), P3 lies to the left (resp. right) of P1 P2
// fn cross3(p1 : &Point, p2 : &Point, p3 : &Point) -> i64 {
//     (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
// }

fn angle(center: &Point, asteroid: &Point) -> f64 {
    // atan2(y,x) Returns the value theta such that -Pi < theta <= Pi, and for some r, x = r cos theta, y = r sin theta.
    let a = (-(asteroid.y - center.y) as f64).atan2((asteroid.x - center.x) as f64);
    // angle w.r.t. up direction
    let a = std::f64::consts::PI / 2.0 - a;
    if a < 0.0 {
        a + 2.0 * std::f64::consts::PI
    } else {
        a
    }
}

fn norm2(p: &Point) -> f64 {
    ((p.x * p.x + p.y * p.y) as f64).sqrt()
}

fn distance(center: &Point, asteroid: &Point) -> f64 {
    norm2(&Point {
        x: center.x - asteroid.x,
        y: center.y - asteroid.y,
    })
}

fn part2(asteroids: &Vec<Point>, center: &Point, nth: usize) -> Point {
    let mut asteroids: Vec<_> = asteroids
        .iter()
        .map(|asteroid| {
            (
                asteroid.clone(),
                angle(&center, &asteroid),
                distance(&center, &asteroid),
            )
        })
        .collect();
    asteroids.sort_by(|(_, t1, _), (_, t2, _)| t1.partial_cmp(t2).unwrap());
    // for a in asteroids.iter() {println!("{:?}", a);};
    let mut i = 0;
    loop {
        i += 1;
        let (mut a, theta, mut da) = asteroids.remove(0);

        // Filter asteroids with same angle, and pick the one with min distance
        while asteroids[0].1 == theta {
            let (b, _, db) = asteroids.remove(0);
            if da < db {
                asteroids.push((b, theta, db))
            } else {
                asteroids.push((a, theta, da));
                a = b;
                da = db
            }
            //   println!("{:?}", asteroids[asteroids.len()-1]);
        }

        // println!("{} {:?}",i,  a);
        if i == nth {
            return a;
        }
    }
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let (asteroids, _) = parse(&contents);
    println!("{}", asteroids.len());

    let (location, max) = asteroids
        .iter()
        .map(|location| (location, number_of_asteroids_in_sight(&asteroids, location)))
        .max_by_key(|&(_, c)| c)
        .unwrap();
    println!("{:?}", max);

    let last = part2(&asteroids, location, 200);
    println!("{:?}", last);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
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

        let (asteroids, _) = parse(puzzle);
        let max = number_of_asteroids_in_sight(&asteroids, &Point { x: 6, y: 3 });
        assert_eq!(max, 41);
    }

    // #[test]
    // fn test_cross3() {
    //     let p1 = Point {x : 0, y : 0};
    //     let p2 = Point {x : 0, y : 1};
    //     let  p3 = Point {x : 1, y : 0};
    //     assert_eq!(cross3(&p1, &p2, &p3), -1);
    //     assert_eq!(cross3(&p1, &p3, &p2), 1);
    //     assert_eq!(cross3(&p3, &p2, &p1), 1);
    // }

    fn norm(mut a: f64) -> f64 {
        let twopi = 2.0 * std::f64::consts::PI;
        while a < 0.0 {
            a += twopi
        }
        while twopi < a {
            a -= twopi
        }
        a
    }

    #[test]
    fn test_part2_basic_angles() {
        let center = Point { x: 0, y: 0 };
        let u = Point { x: 0, y: -1 };
        assert_eq!(angle(&center, &u), 0.0);
        let r = Point { x: 1, y: 0 };
        assert_eq!(angle(&center, &r), std::f64::consts::PI / 2.0);
        let d = Point { x: 0, y: 1 };
        assert_eq!(angle(&center, &d), std::f64::consts::PI);
        let l = Point { x: -1, y: 0 };
        assert_eq!(angle(&center, &l), 3.0 * std::f64::consts::PI / 2.0);
    }

    #[test]
    fn test_part2_angles() {
        let center = Point { x: 11, y: 13 };
        let u = Point { x: 11, y: 12 };
        assert_eq!(angle(&center, &u), 0.0);
        let d = Point { x: 11, y: 14 };
        assert_eq!(angle(&center, &d), std::f64::consts::PI);
    }

    #[test]
    fn test_part2_large_example() {
        let t = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";
        let center = Point { x: 11, y: 13 };
        let (mut asteroids, _) = parse(t);
        asteroids.retain(|&a| a != center);
        assert_eq!(part2(&asteroids, &center, 1), Point { x: 11, y: 12 });
        assert_eq!(part2(&asteroids, &center, 2), Point { x: 12, y: 1 });
        assert_eq!(part2(&asteroids, &center, 10), Point { x: 12, y: 8 });
        assert_eq!(part2(&asteroids, &center, 20), Point { x: 16, y: 0 });
        assert_eq!(part2(&asteroids, &center, 100), Point { x: 10, y: 16 })
    }

    #[test]
    fn test_part2_small_example() {
        let t = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##";
        let (asteroids, center) = parse(t);
        let center = center.unwrap();
        assert_eq!(part2(&asteroids, &center, 9), Point { x: 15, y: 1 });
        assert_eq!(part2(&asteroids, &center, 18), Point { x: 4, y: 4 });
    }
}
