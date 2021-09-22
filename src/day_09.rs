use std::collections::VecDeque;

// The `front` of the Deque corresponds to the current marble, and indicates the
// clockwise direction. The back of the Dequeue correspond to the coutner
// clockwise direction
struct T {
    circle: VecDeque<usize>,
    score: Vec<usize>,
    player: usize,
    marble: usize,
}

impl T {
    fn new(players: usize) -> T {
        let mut circle = VecDeque::new();
        circle.push_back(0);

        T {
            circle,
            score: (0..players).map(|_i| 0).collect(),
            player: 0,
            marble: 1,
        }
    }

    #[allow(dead_code)]
    fn debug(&self) {
        let mut buf = String::new();
        buf.push_str(&format!("[{}] ", self.player + 1));
        let mut c = self.circle.clone();
        let current = c.pop_front().unwrap();
        buf.push_str(&format!("({}) ", current));
        while !c.is_empty() {
            let n = c.pop_front().unwrap();
            buf.push_str(&format!("{} ", n));
        }
        println!("{}", buf)
    }

    fn play(&mut self) {
        let marble = self.marble;
        if marble % 23 != 0 {
            let n = self.circle.pop_front().unwrap();
            self.circle.push_back(n);
            let n = self.circle.pop_front().unwrap();
            self.circle.push_back(n);
            self.circle.push_front(marble)
        } else {
            self.score[self.player] += marble;
            for _i in 0..6 {
                let n = self.circle.pop_back().unwrap();
                self.circle.push_front(n);
            }
            let n = self.circle.pop_back().unwrap();
            self.score[self.player] += n;
        };
        self.player = (self.player + 1) % self.score.len();
        self.marble += 1
    }
}

fn part1(players: usize, last_marble: usize) -> usize {
    let mut t = T::new(players);
    while t.marble != last_marble + 1 {
        t.play()
    }
    t.score.into_iter().max().unwrap()
}

pub fn run(filename: &str) {
    // 428 players; last marble is worth 72061 points
    let content = std::fs::read_to_string(filename).unwrap();
    let words: Vec<_> = content.split(' ').collect();
    let players: usize = words[0].parse().unwrap();
    let last_marble: usize = words[6].parse().unwrap();
    println!("{}", part1(players, last_marble));
    println!("{}", part1(players, 100 * last_marble))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples_part1() {
        assert_eq!(part1(9, 25), 32);
        assert_eq!(part1(10, 1618), 8317);
        assert_eq!(part1(13, 7999), 146373);
    }
}
