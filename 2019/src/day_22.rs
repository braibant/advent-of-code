//
#[derive(Debug, Clone, Copy)]
enum Technique {
    Deal,
    Cut(isize),
    DealWithIncrement(usize),
}

fn parse(s: &str) -> Vec<Technique> {
    let mut techniques = Vec::new();
    for line in s.split('\n') {
        let t = if line == "deal into new stack" {
            Technique::Deal
        } else if let Ok(n) = scan_fmt::scan_fmt!(line, "deal with increment {}", usize) {
            Technique::DealWithIncrement(n)
        } else if let Ok(n) = scan_fmt::scan_fmt!(line, "cut {}", isize) {
            Technique::Cut(n)
        } else {
            panic!("Cannot parse technique {}", line)
        };
        techniques.push(t)
    }
    techniques
}

// Let's implement a deck of card as an array A, such that A[i] denotes the
// index of the card after i. When a deck is in factory order, A[i] = (i+1) %
// A.len(). We also need to remember the index of the first card in the deck.

struct T {
    len: usize,
    next: Vec<usize>,
    first: usize,
}

impl T {
    fn new(len: usize) -> T {
        let mut next = Vec::new();
        for i in 1..(len + 1) {
            next.push(i)
        }
        T {
            next,
            first: 0,
            len: len,
        }
    }

    #[allow(dead_code)]
    fn to_vec(&self) -> Vec<usize> {
        let mut v = Vec::new();
        let mut x = self.first;
        for _i in 0..self.len {
            v.push(x);
            x = self.next[x];
        }
        v
    }

    #[allow(dead_code)]
    fn invariant(&self) -> bool {
        self.next.iter().all(|&n| n <= self.len)
            && self.first < self.len
            && self.next.iter().any(|&n| n == self.len)
    }

    fn deal(&mut self) {
        let mut prev = self.len;
        let mut this = self.first;
        for _i in 0..(self.len) {
            let next = self.next[this];
            self.next[this] = prev;
            prev = this;
            this = next;
        }
        self.first = prev;
    }

    fn cut(&mut self, mut n: isize) {
        let first = self.first;
        // Let's only consider cuts with a positive number of cards
        n = n.rem_euclid(self.len as isize);
        let mut prev = None;
        let mut this = self.first;
        for _i in 0..n {
            prev = Some(this);
            this = self.next[this]
        }
        self.next[prev.unwrap()] = self.len;
        self.first = this;
        // We now find the last card in the initial deck
        while self.next[this] != self.len {
            this = self.next[this]
        }
        self.next[this] = first
    }

    fn deal_with_increment(&mut self, n: usize) {
        let mut v = Vec::with_capacity(self.len);
        v.resize(self.len, self.len + 1);

        let mut i: usize = 0;
        let mut this = self.first;
        for _count in 0..self.len {
            v[i] = this;
            i = (i + n) % self.len;
            this = self.next[this]
        }
        // Enforce invariant on the end result
        assert!(v.iter().all(|&n| n <= self.len));

        // Rebuild the deck according to the order in [v]
        self.first = v[0];
        this = v[0];
        for i in 0..self.len - 1 {
            self.next[this] = v[i + 1];
            this = v[i + 1]
        }
        self.next[this] = self.len
    }

    fn apply(&mut self, technique: Technique) {
        match technique {
            Technique::Deal => self.deal(),
            Technique::Cut(n) => self.cut(n),
            Technique::DealWithIncrement(n) => self.deal_with_increment(n),
        }
    }

    fn find_card(&self, card: usize) -> Option<usize> {
        let mut this = self.first;
        let mut count = 0;
        while count < self.len {
            if this == card {
                return Some(count);
            } else {
                this = self.next[this];
                count += 1;
            }
        }
        return None;
    }
}

fn mpow(x: i128, mut n: i128, modulo: i128) -> i128 {
    if modulo == 0 {
        return 1;
    } else {
        let mut result = 1;
        let mut x = x.rem_euclid(modulo);
        while n > 0 {
            if n % 2 == 1 {
                result = (result * x).rem_euclid(modulo)
            };
            n = n / 2;
            x = (x * x).rem_euclid(modulo)
        }
        return result;
    }
}

fn minv(x: i128, modulo: i128) -> i128 {
    mpow(x, modulo - 2, modulo)
}

struct Transform {
    size: i128,
    offset: i128, // the first value in the deck,
    increment: i128,
}

impl Transform {
    fn ident(size: i128) -> Transform {
        Transform {
            size,
            offset: 0,
            increment: 1,
        }
    }

    fn new(size: i128, increment: i128, offset: i128) -> Transform {
        Transform {
            size,
            increment: increment.rem_euclid(size),
            offset: offset.rem_euclid(size),
        }
    }

    fn deal(&self) -> Transform {
        Transform::new(self.size, -self.increment, self.offset - self.increment)
    }

    fn cut(&self, n: isize) -> Transform {
        Transform::new(
            self.size,
            self.increment,
            self.offset + (self.increment * (n as i128)).rem_euclid(self.size),
        )
    }

    fn deal_with_increment(&self, n: usize) -> Transform {
        Transform::new(
            self.size,
            self.increment * minv(n as i128, self.size),
            self.offset,
        )
    }

    fn pow(&self, n: i128) -> Transform {
        Transform::new(
            self.size,
            mpow(self.increment, n, self.size),
            (self.offset * (1 - mpow(self.increment, n, self.size)).rem_euclid(self.size))
                .rem_euclid(self.size)
                * minv(1 - self.increment, self.size),
        )
    }

    fn to_vec(&self) -> Vec<i128> {
        let mut result = Vec::new();
        let mut i = self.offset;
        for _ in 0..self.size {
            result.push(i);
            i = (i + self.increment) % self.size
        }
        result
    }

    fn get(&self, n: i128) -> i128 {
        (self.offset + self.increment * n).rem_euclid(self.size)
    }

    fn find_card(&self, n: i128) -> Option<i128> {
        if 0 <= n && n < self.size {
            let mut this = self.offset;
            let mut i = 0;
            loop {
                if this == n {
                    return Some(i);
                };
                this = (this + self.increment).rem_euclid(self.size);
                i += 1
            }
        } else {
            return None;
        }
    }
}

fn part1(techniques: &[Technique]) {
    let mut deck = T::new(10007);
    techniques.iter().for_each(|&t| deck.apply(t));
    println!("{:?}", deck.find_card(2019));
}

fn part1_bis(techniques: &[Technique]) {
    const DECK_SIZE: i128 = 10_007;
    let mut aff = Transform::ident(DECK_SIZE);
    for &t in techniques.iter() {
        aff = match t {
            Technique::Deal => aff.deal(),
            Technique::Cut(i) => aff.cut(i),
            Technique::DealWithIncrement(i) => aff.deal_with_increment(i),
        };
    }
    println!("{:?}", aff.find_card(2019));
}
fn part2(techniques: &[Technique]) {
    const DECK_SIZE: i128 = 119_315_717_514_047;
    const REPETITION_COUNT: i128 = 101_741_582_076_661;

    let mut acc = Transform::ident(DECK_SIZE);
    for &t in techniques.iter() {
        acc = match t {
            Technique::Deal => acc.deal(),
            Technique::Cut(i) => acc.cut(i),
            Technique::DealWithIncrement(i) => acc.deal_with_increment(i),
        };
    }
    let acc = acc.pow(REPETITION_COUNT);
    println!("{}", acc.get(2020))
}

pub fn run(filename: &str) {
    let content = std::fs::read_to_string(filename).unwrap();
    let techniques = parse(&content);
    part1(&techniques);
    part1_bis(&techniques);
    part2(&techniques)
}

#[cfg(test)]
mod tests {

    mod part1 {
        use super::super::*;

        #[test]
        fn test_new() {
            let deck = T::new(5);
            assert_eq!(deck.to_vec(), vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn test_deal_1() {
            let mut deck = T::new(5);
            assert_eq!(deck.to_vec(), vec![0, 1, 2, 3, 4]);
            deck.deal();
            assert_eq!(deck.to_vec(), vec![4, 3, 2, 1, 0]);
        }

        #[test]
        fn test_cut_1() {
            let mut deck = T::new(5);
            assert_eq!(deck.to_vec(), vec![0, 1, 2, 3, 4]);
            deck.cut(2);
            assert_eq!(deck.to_vec(), vec![2, 3, 4, 0, 1]);
        }

        #[test]
        fn test_cut_2() {
            let mut deck = T::new(5);
            assert_eq!(deck.to_vec(), vec![0, 1, 2, 3, 4]);
            deck.cut(-2);
            assert_eq!(deck.to_vec(), vec![3, 4, 0, 1, 2]);
        }

        #[test]
        fn test_deal_with_increment_1() {
            let mut deck = T::new(10);
            deck.deal_with_increment(3);
            assert_eq!(deck.to_vec(), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
        }

        #[test]
        fn part_1_example_1() {
            let mut deck = T::new(10);
            deck.deal_with_increment(7);
            deck.deal();
            deck.deal();
            assert_eq!(deck.to_vec(), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7])
        }

        #[test]
        fn part_1_example_2() {
            let mut deck = T::new(10);
            deck.cut(6);
            deck.deal_with_increment(7);
            deck.deal();
            assert_eq!(deck.to_vec(), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6])
        }

        #[test]
        fn part_1_example_3() {
            let mut deck = T::new(10);
            deck.deal();
            deck.cut(-2);
            deck.deal_with_increment(7);
            deck.cut(8);
            deck.cut(-4);
            deck.deal_with_increment(7);
            deck.cut(3);
            deck.deal_with_increment(9);
            deck.deal_with_increment(3);
            deck.cut(-1);
            assert_eq!(deck.to_vec(), vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6])
        }
    }

    use super::mpow;
    #[test]
    fn test_pow() {
        assert_eq!(mpow(1, 10, 1000), 1);
        assert_eq!(mpow(2, 10, 1000), 24);
        assert_eq!(mpow(3, 17, 1000), 163);
    }
    mod part2 {
        use super::super::*;
        #[test]
        fn test_deal_1() {
            let deck = Transform::ident(5);
            let deck = deck.deal();
            assert_eq!(deck.to_vec(), vec![4, 3, 2, 1, 0]);
        }

        #[test]
        fn test_cut_1() {
            let deck = Transform::ident(7);
            let deck = deck.cut(2);
            assert_eq!(deck.to_vec(), vec![2, 3, 4, 5, 6, 0, 1]);
        }

        #[test]
        fn test_cut_2() {
            let deck = Transform::ident(10).cut(-4);
            assert_eq!(deck.to_vec(), vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
        }

        #[test]
        fn test_deal_with_increment_1() {
            let deck = Transform::ident(11).deal_with_increment(3);
            assert_eq!(deck.to_vec(), vec![0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7]);
        }
    }
}
