use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_bsp(s: &str, high: char) -> usize {
    let mut r = 0;
    let len = s.len();
    for (index, char) in s.chars().enumerate() {
        let mask = 1 << (len - index - 1);
        if char == high {
            r |= mask
        };
    }
    r
}

fn parse_seat_id(s: &str) -> usize {
    let row = parse_bsp(&s[0..7], 'B');
    let col = parse_bsp(&s[7..10], 'R');
    row * 8 + col
}

pub fn run(filename: String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut max_seen = 0;
    let mut min_seen = 127 * 8;
    // We actually don't know how many seats the plane has!
    // However, we can over-approximate it using the number of bits in the seat id.
    let mut bitmap: [bool; 127 * 8] = [false; 127 * 8];
    for line in reader.lines() {
        let line = line.unwrap();
        let seat_id = parse_seat_id(&line);
        bitmap[seat_id] = true;
        max_seen = usize::max(max_seen, seat_id);
        min_seen = usize::min(min_seen, seat_id);
    }
    println!("{}", max_seen);
    for (index, value) in bitmap.iter().enumerate() {
        if !value && min_seen <= index && index <= max_seen {
            println!("{}", index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day5_example1() {
        assert_eq!(parse_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(parse_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(parse_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(parse_seat_id("BBFFBBFRLL"), 820);
    }
}
