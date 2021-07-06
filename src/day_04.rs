use std::time::{Duration, Instant};

// In this problem, we consider passwords which are 6 digits decimal numbers.
// We need to do two things with those numbers: iter over a range, and select the digits of an individual number. We could use two different representations: a) represent the numbers as vectors of digits (which makes the indexing operation trivial, at the expanse of having to implement the range iteration) or iter over an integer range, and convert integers in the given range to vectors of digits. We chose the later here.

mod v1 {
    use lazy_static::lazy_static;

    const N: usize = 6;
    type T = u32;
    lazy_static! {
        static ref powers_of_10: [T; N] = {
            let mut t = [0; N];
            let mut n = 1;
            for i in 0..N {
                t[i] = n;
                n = n * 10;
            }
            t
        };
    }

    fn digit(n: u32, i: usize) -> u8 {
        ((n / powers_of_10[i]) % 10) as u8
    }

    // N = ABCDEF is represented as the vector [F, E, D, C, B, A]
    pub fn create(n: T) -> Vec<u8> {
        let mut v = vec![];
        for i in 0..N {
            v.push(digit(n, i))
        }
        v
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test1() {
            assert_eq!(create(123456), vec![6, 5, 4, 3, 2, 1])
        }
    }
}

mod v2 {
    // Passwords are represented as vectors of bytes. The number `n = \sum_i a_i * 10 ^ i` is represented as [a_0, a_1, ...]
    type T = Vec<u8>;

    fn incr(n: &mut T) {
        let mut cur = 0;
        let mut carry = 1;
        while carry != 0 {
            let d = n[cur];
            if d < 9 {
                n[cur] += 1;
                carry = 0;
            } else {
                n[cur] = 0;
                cur += 1;
            }
        }
    }

    pub struct Range {
        next: T,
        end: T,
    }

    pub fn range(start: &T, end: &T) -> Range {
        Range {
            next: start.clone(),
            end: end.clone(),
        }
    }

    impl Iterator for Range {
        type Item = Vec<u8>;
        fn next(&mut self) -> Option<Self::Item> {
            if self.next == self.end {
                None
            } else {
                let next = self.next.clone();
                incr(&mut self.next);
                Some(next)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        #[test]
        fn test_incr() {
            let mut n = vec![1, 0, 0];
            incr(&mut n);
            assert_eq!(n, vec![2, 0, 0]);
            let mut n = vec![9, 0, 0];
            incr(&mut n);
            assert_eq!(n, vec![0, 1, 0]);
        }

        #[test]
        fn test_create_incr() {
            let mut n = v1::create(9);
            incr(&mut n);
            assert_eq!(n, vec![0, 1, 0, 0, 0, 0]);
        }
    }
}

fn check_password1(n: &Vec<u8>) -> bool {
    let mut consecutive_digits_equal = false;
    let mut digits_decrease_or_equal = true;
    for i in 1..n.len() {
        consecutive_digits_equal = consecutive_digits_equal || n[i] == n[i - 1];
        digits_decrease_or_equal = digits_decrease_or_equal && n[i - 1] >= n[i];
    }

    consecutive_digits_equal && digits_decrease_or_equal
}

// For part 2, we need to check an extra condition: there must be a group of consecutive equal digits of length exactly 2. There are a couple of ways to do that: by computing the length of each group of repeating digits, then checking the existence of one of size 2; or simply by computing the number of times each digit is repeated and exploiting the fact that the "digits increase or are equal" property entail that identical digits must be part of the same group
fn check_digit_groups(n: &Vec<u8>) -> bool {
    let mut digits: [u32; 10] = [0; 10];
    for &digit in n.iter() {
        digits[digit as usize] += 1
    }

    digits.iter().any(|&tally| tally == 2)
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let range: Vec<_> = contents.split("-").collect();
    let low: u32 = range[0].parse().unwrap();
    let high: u32 = range[1].parse().unwrap();

    let now = Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    for i in low..high + 1 {
        let n = v1::create(i);
        if check_password1(&n) {
            part1 += 1;

            if check_digit_groups(&n) {
                part2 += 1;
            }
        }
    }
    println!("{}", part1);
    println!("{}", part2);

    println!("v1 : {:?}", now.elapsed());

    let now = Instant::now();
    let mut part1 = 0;
    let mut part2 = 0;
    for n in v2::range(&v1::create(low), &v1::create(high)) {
        if check_password1(&n) {
            part1 += 1;

            if check_digit_groups(&n) {
                part2 += 1;
            }
        }
    }
    println!("{}", part1);
    println!("{}", part2);

    println!("v2 : {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let n = v1::create(111122);
        assert_eq!(check_digit_groups(&n), true);
        let n = v1::create(123444);
        assert_eq!(check_digit_groups(&n), false)
    }
}
