use std::fs::File;
use std::io::{BufRead, BufReader};

fn bit(c: char) -> usize {
    assert!(c.is_ascii_alphabetic());
    ((c as u8) - b'a') as usize
}

fn parse(s: &str) -> usize {
    let mut mask = 0;
    for c in s.chars() {
        mask |= 1 << bit(c)
    }
    mask
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut acc_anyone: usize = 0;
    let mut result_anyone: u32 = 0;
    let mut acc_everyone: usize = !0;
    let mut result_everyone: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            result_everyone += acc_everyone.count_ones();
            result_anyone += acc_anyone.count_ones();
            acc_everyone = !0;
            acc_anyone = 0;
        } else {
            acc_anyone |= parse(&line);
            acc_everyone &= parse(&line);
        }
    }
    if acc_anyone != 0 || acc_everyone != 0 {
        result_everyone += acc_everyone.count_ones();
        result_anyone += acc_anyone.count_ones();
    };
    println!("{}", result_anyone);
    println!("{}", result_everyone)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bit_fiddling() {
        assert_eq!(bit('a'), 0);
        assert_eq!(bit('c'), 2);
        assert_eq!(parse("abc"), 7);
    }
}
