fn fuel(mass: u64) -> u64 {
    if mass <= 6 {
        0
    } else {
        let req = mass / 3 - 2;
        req + fuel(req)
    }
}

pub fn run(filename: &str) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let modules: Vec<_> = contents
        .split('\n')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut part1 = 0;
    for mass in modules.iter() {
        let fuel = mass / 3 - 2;
        part1 += fuel;
    }
    println!("{}", part1);

    let mut part2 = 0;
    for &mass in modules.iter() {
        part2 += fuel(mass);
    }
    println!("{}", part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(fuel(100756), 50346)
    }
}
