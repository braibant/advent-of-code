use super::intcode::*;

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let program: Vec<_> = contents
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // part 1
    let mut p1 = program.clone();
    p1[1] = 12;
    p1[2] = 2;
    let mut input = vec![];

    execute(&mut p1, &mut input);
    println!("{}", p1[0]);

    // part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let mut p2 = program.clone();
            p2[1] = noun;
            p2[2] = verb;

            execute(&mut p2, &mut input);
            if p2[0] == 19690720 {
                println!("{}", 100 * noun + verb)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let mut p = vec![1, 0, 0, 0, 99];
        let mut input = vec![];
        let mut output = vec![];
        execute(&mut p, &mut input);
        assert_eq!(p, vec![2, 0, 0, 0, 99])
    }

    #[test]
    fn test_2() {
        let mut p = vec![2, 3, 0, 3, 99];
        let mut input = vec![];
        let mut output = vec![];
        execute(&mut p, &mut input);
        assert_eq!(p, vec![2, 3, 0, 6, 99])
    }

    #[test]
    fn test_3() {
        let mut p = vec![2, 4, 4, 5, 99, 0];
        let mut input = vec![];
        let mut output = vec![];
        execute(&mut p, &mut input);
        assert_eq!(p, vec![2, 4, 4, 5, 99, 9801])
    }
}
