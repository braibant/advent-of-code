use lazy_static::lazy_static;

// In this problem, we consider passwords which are 6 digits decimal numbers.
// We need to do two things with those numbers: iter over a range, and select the digits of an individual number. We could use two different representations: a) represent the numbers as vectors of digits (which makes the indexing operation trivial, at the expanse of having to implement the range iteration) or iter over an integer range, and convert integers in the given range to vectors of digits. We chose the later here.
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

// N = ABCDEF is represented as the vector [A, B, C, D, E, F]
fn create(n: T) -> Vec<u8> {
    let mut v = vec![];
    for i in 0..N {
        v.push(digit(n, i))
    }
    v.reverse();
    v
}

fn check_password1(n: &Vec<u8>) -> bool {
    let mut consecutive_digits_equal = false;
    let mut digits_increase_or_equal = true;
    for i in 1..n.len() {
        consecutive_digits_equal = consecutive_digits_equal || n[i] == n[i - 1];
        digits_increase_or_equal = digits_increase_or_equal && n[i - 1] <= n[i];
    }

    consecutive_digits_equal && digits_increase_or_equal
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

    let mut part1 = 0;
    for i in low..high + 1 {
        if check_password1(&create(i)) {
            part1 += 1;
        }
    }
    println!("{}", part1);

    let mut part2 = 0;
    for i in low..high + 1 {
        let n = create(i);
        if check_password1(&n) && check_digit_groups(&n) {
            part2 += 1;
        }
    }
    println!("{}", part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(create(123456), vec![1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn test2() {
        let n = create(111122);
        assert_eq!(check_digit_groups(&n), true);
        let n = create(123444);
        assert_eq!(check_digit_groups(&n), false)
    }
}
