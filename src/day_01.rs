use std::collections::HashSet;

fn parse(s: &str) -> Vec<i64> {
    s.split('\n')
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let i: i64 = l
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Could not parse line '{}' as number", l));
                Some(i)
            }
        })
        .collect()
}

fn part1(input: &[i64]) -> i64 {
    input.iter().sum()
}

fn part2(input: &[i64]) -> i64 {
    let mut i = 0;
    let mut freq = 0;
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&freq) {
            return freq;
        } else {
            seen.insert(freq);
            freq += input[i % input.len()];
            i += 1
        }
    }
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).expect("Cannot read file");
    let input = parse(&content);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {

    mod part2 {
        use super::super::*;
        #[test]
        fn examples() {
            assert_eq!(0, part2(&[1, -1]));
            assert_eq!(10, part2(&[3, 3, 4, -2, -4]));
            assert_eq!(5, part2(&[-6, 3, 8, 5, -6]));
            assert_eq!(14, part2(&[7, 7, -2, -7, -4]));
        }
    }
}
