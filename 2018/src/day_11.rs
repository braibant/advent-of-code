use std::collections::HashMap;

fn power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = (rack_id * y + grid_serial_number) * rack_id;
    ((power_level % 1000) / 100) - 5
}

fn total_power(x: i32, y: i32, size: i32, grid_serial_number: i32) -> i32 {
    (x..x + size)
        .flat_map(|x| (y..y + size).map(move |y| (x, y)))
        .map(|(x, y)| power_level(x, y, grid_serial_number))
        .sum()
}

fn part1(grid_serial_number: i32) -> (i32, i32) {
    let n = 298;
    let (x, y, _) = (1..=n)
        .flat_map(|x| (1..=n).map(move |y| (x, y)))
        .map(|(x, y)| (x, y, total_power(x, y, 3, grid_serial_number)))
        .max_by_key(|(_, _, k)| *k)
        .unwrap();
    (x, y)
}

// fn part2(grid_serial_number: i32) -> (i32, i32, i32) {
//     let mut d = HashMap::new();
//     // Initialize with squares of size 1
//     for x in 1..=300 {
//         for y in 1..=300 {
//             d.insert((x, y, 1), power_level(x, y, grid_serial_number));
//         }
//     }

//     for s in 2..=300 {
//         println!("{}", s);
//         for x in 1..=300 {
//             for y in 1..=300 {
//                 if x + s <= 300 && y + s <= 300 {
//                     let &l = d.get(&(x, y, s - 1)).unwrap();
//                     let fringe: i32 = (y..y + s)
//                         .map(|y| (x + s - 1, y))
//                         .chain((x..x + s - 1).map(|x| (x, y + s - 1)))
//                         .map(|(x, y)| power_level(x, y, grid_serial_number))
//                         .sum();
//                     d.insert((x, y, s), fringe + l);
//                 }
//             }
//         }
//     }

//     println!("{}", d.len());
//     let (&(x, y, s), _) = d.iter().max_by_key(|(_k, v)| *v).unwrap();
//     (x, y, s)
// }

fn part2(grid_serial_number: i32) -> (i32, i32, i32) {
    let mut d = HashMap::new();
    let mut best_level = i32::MIN;
    let mut best_position = None;
    // Initialize with squares of size 1
    for x in 1..=300 {
        for y in 1..=300 {
            let p = power_level(x, y, grid_serial_number);
            d.insert((x, y), p);
            if p > best_level {
                best_position = Some((x, y, 1));
                best_level = p
            }
        }
    }

    // Note that the average power level is around -0.5, which means that the
    // expected value of the total power level goes down as the size increases.
    // Hence, we can reduce the max value of s here.
    for s in 2..=150 {
        for x in 1..=300 {
            for y in 1..=300 {
                if x + s <= 300 && y + s <= 300 {
                    let &l = d.get(&(x, y)).unwrap();
                    let fringe: i32 = (y..y + s)
                        .map(|y| (x + s - 1, y))
                        .chain((x..x + s - 1).map(|x| (x, y + s - 1)))
                        .map(|(x, y)| power_level(x, y, grid_serial_number))
                        .sum();
                    let p = fringe + l;
                    d.insert((x, y), p);
                    if p > best_level {
                        best_position = Some((x, y, s));
                        best_level = p
                    }
                }
            }
        }
    }
    best_position.unwrap()
}

fn average(grid_serial_number: i32) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;
    for x in 1..=300 {
        for y in 1..=300 {
            sum += power_level(x, y, grid_serial_number) as f64;
            count += 1
        }
    }
    sum / count as f64
}

pub fn run(filename: &str) {
    let mut contents = std::fs::read_to_string(filename).unwrap();
    if contents.ends_with('\n') {
        contents.pop();
    };
    let grid_serial_number: i32 = contents.parse().unwrap();
    println!("Average power level: {}", average(grid_serial_number));
    let (x, y) = part1(grid_serial_number);
    println!("{},{}", x, y);
    let (x, y, s) = part2(grid_serial_number);
    println!("{},{},{}", x, y, s);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_power_level() {
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_example_part_1() {
        let grid_serial_number = 42;
        assert_eq!(power_level(21, 61, grid_serial_number), 4);
        assert_eq!(power_level(22, 61, grid_serial_number), 3);
        assert_eq!(power_level(23, 61, grid_serial_number), 3);

        assert_eq!(total_power(21, 61, 3, grid_serial_number), 30);
        assert_eq!(part1(grid_serial_number), (21, 61));
    }

    #[test]
    fn test_example_part_2() {
        assert_eq!(total_power(90, 269, 16, 18), 113);
        assert_eq!(total_power(232, 251, 12, 42), 119);
    }
}
