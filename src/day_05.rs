fn matching(a: char, b: char) -> bool {
    if a.to_ascii_uppercase() == b.to_ascii_uppercase() {
        (a.is_ascii_lowercase() && b.is_ascii_uppercase())
            || (a.is_ascii_uppercase() && b.is_ascii_lowercase())
    } else {
        false
    }
}

// The goal of part 1 is to update a (long) string according to some rule
// (adjacent identical characters with different capitalisation are removed.)
// Here, we need to update the string to remove pairs of adjacent letters,
// potentially creating new opportunities for deletion. Let's consider a
// representation which allows for removal around a "cursor" in O(1) by having
// the left hand side of the string represented as a Vector (in natural order),
// and the right hand size of the string represented as a vector (in reversed
// order). Let's have the invariant that the left hand side has no reaction
// site, then check if the last element of the left hand side and the last
// element of the right hand side match. If yes, both are removed. If not, the
// right hand side element is pushed on the left hand side, preserving our
// invariant.

#[derive(Clone)]
struct T {
    left: Vec<char>,
    right: Vec<char>,
}

impl T {
    fn new(s: &str) -> T {
        // Initialize
        let mut right: Vec<_> = s.chars().collect();
        right.reverse();
        T {
            left: Vec::new(),
            right,
        }
    }

    fn reset(&mut self) {
        while !self.left.is_empty() {
            let t = self.left.pop().unwrap();
            self.right.push(t)
        }
    }

    fn len(&self) -> usize {
        self.left.len() + self.right.len()
    }

    fn filter(&mut self, c: char) {
        self.reset();
        let c = c.to_ascii_uppercase();
        self.right.retain(|x| x.to_ascii_uppercase() != c)
    }

    fn react(self: &mut T) {
        self.reset();
        while !self.right.is_empty() {
            if self.left.is_empty() {
                let x = self.right.pop().unwrap();
                self.left.push(x)
            } else {
                let l = self.left.pop().unwrap();
                let r = self.right.pop().unwrap();
                if matching(l, r) {
                } else {
                    self.left.push(l);
                    self.left.push(r);
                }
            }
        }
    }
}

fn part1(s: &str) -> usize {
    let mut t = T::new(s);
    t.react();
    t.len()
}

// An interesting property for part 2 is that erasure of a given type and the
// general reaction rule is confluent (i.e., brings us to the same string)
fn part2(s: &str) -> usize {
    let mut t = T::new(s);
    t.react();
    t.reset();
    let (_c, len) = ('a'..'z')
        .map(|c| {
            let mut t = t.clone();
            t.filter(c);
            t.react();
            (c, t.len())
        })
        .min_by_key(|&(_c, len)| len)
        .unwrap();
    len
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    println!("{}", part1(&content));
    println!("{}", part2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert!(react('a', 'A'));
        assert!(react('A', 'a'));
        assert!(!react('b', 'a'));
        assert!(!react('a', 'a'));
        assert!(!react('B', 'B'));
        let s = "dabAcCaCBAcCcaDA";
        assert_eq!(part1(s), 10);
        let s = "abBA";
        assert_eq!(part1(s), 0);
        let s = "abAB";
        assert_eq!(part1(s), 4);
        let s = "aabAAB";
        assert_eq!(part1(s), 6);
    }
}
