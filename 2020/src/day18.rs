use std::fs::File;
use std::io::{BufRead, BufReader};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::many0,
    IResult,
};

#[derive(Clone, Debug)]
enum T {
    Int(u64),
    Add,
    Mul,
    Lpar,
    Rpar,
}

fn integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn token(input: &str) -> IResult<&str, T> {
    let mut tokens = alt((
        map(tag("+"), |_| T::Add),
        map(tag("*"), |_| T::Mul),
        map(tag("("), |_| T::Lpar),
        map(tag(")"), |_| T::Rpar),
        map(integer, |n| T::Int(n)),
    ));
    tokens(input)
}

fn parse(input: &str) -> IResult<&str, Vec<T>> {
    many0(token)(input)
}

fn eval1(input: &[T], mut i: usize) -> (u64, usize) {
    let mut nums = Vec::new();
    let mut op = T::Add;
    while i < input.len() {
        match input[i] {
            T::Lpar => {
                let (n, last) = eval1(input, i + 1);
                nums.push(n);
                i = last
            }
            T::Rpar => break,
            T::Add => op = T::Add,
            T::Mul => op = T::Mul,
            T::Int(n) => nums.push(n),
        };
        if nums.len() == 2 {
            let a = nums.pop().unwrap();
            let b = nums.pop().unwrap();
            match op {
                T::Mul => nums.push(a * b),
                T::Add => nums.push(a + b),
                _ => panic!("Invalid"),
            }
        };
        i += 1;
    }

    return (nums.pop().unwrap(), i);
}

fn eval2(input: &[T], mut i: usize) -> (u64, usize) {
    let mut nums = Vec::new();
    let mut acc = 0;
    while i < input.len() {
        match input[i] {
            T::Lpar => {
                let (n, last) = eval2(input, i + 1);
                acc += n;
                i = last
            }
            T::Rpar => break,
            T::Mul => {
                nums.push(acc);
                acc = 0
            }
            T::Add => {}
            T::Int(n) => acc += n,
        };
        i += 1;
    }
    nums.push(acc);
    return (nums.iter().product(), i);
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.replace(" ", "");
        let (_, v) = parse(&line).unwrap();
        let (result, _last) = eval1(&v, 0);
        part1 += result;
        let (result, _last) = eval2(&v, 0);
        part2 += result
    }

    println!("{}", part1);

    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run1(s: &str) -> u64 {
        let s = s.replace(" ", "");
        let (_, v) = parse(&s).unwrap();
        let (result, _last) = eval1(&v, 0);
        result
    }

    fn run2(s: &str) -> u64 {
        let s = s.replace(" ", "");
        let (_, v) = parse(&s).unwrap();
        let (result, _last) = eval2(&v, 0);
        result
    }

    #[test]
    fn test0() {
        assert_eq!(run1("17 * 3"), 51)
    }
    #[test]
    fn test1() {
        assert_eq!(run1("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn test2() {
        assert_eq!(
            run1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        )
    }

    #[test]
    fn test3() {
        assert_eq!(run2("1 + 2 * 3 + 4 * 5 + 6"), 231)
    }

    #[test]
    fn test4() {
        assert_eq!(run2("1 + (2 * 3) + 4 * 5 + 6"), 121)
    }
}
