fn part1(n: usize) -> Vec<u8> {
    let mut scores: Vec<u8> = vec![3, 7];
    let mut pos1 = 0;
    let mut pos2 = 1;

    while scores.len() < n + 10 {
        // let mut buf = String::new();

        // for i in 0..scores.len() {
        //     if i == pos1 && i == pos2 {
        //         buf.push_str(&format!("<{}>", scores[i]))
        //     } else if i == pos1 {
        //         buf.push_str(&format!("({})", scores[i]));
        //     } else if i == pos2 {
        //         buf.push_str(&format!("[{}]", scores[i]));
        //     } else {
        //         buf.push_str(&format!(" {} ", scores[i]));
        //     }
        // }

        // println!("{:?}", buf);
        let score_elf1 = scores[pos1];
        let score_elf2 = scores[pos2];
        let sum = score_elf1 + score_elf2;
        if sum >= 10 {
            scores.push(sum / 10);
        };
        scores.push(sum % 10);
        pos1 = (pos1 + score_elf1 as usize + 1) % scores.len();
        pos2 = (pos2 + score_elf2 as usize + 1) % scores.len();
    }
    scores[n..n + 10].to_vec()
}

fn part2(mut n: usize) -> usize {
    let mut target: Vec<u8> = vec![];
    while n > 0 {
        target.push((n % 10) as u8);
        n /= 10;
    }
    target.reverse();

    let mut scores: Vec<u8> = vec![3, 7];
    let mut pos1 = 0;
    let mut pos2 = 1;

    let n = target.len();

    loop {
        if scores.len() >= n && scores[(scores.len() - n)..] == target[..] {
            return scores.len() - n;
        }

        if scores.len() >= n + 1 && scores[(scores.len() - 1 - n)..scores.len() - 1] == target[..] {
            return scores.len() - 1 - n;
        }

        let score_elf1 = scores[pos1];
        let score_elf2 = scores[pos2];
        let sum = score_elf1 + score_elf2;
        if sum >= 10 {
            scores.push(sum / 10);
        };
        scores.push(sum % 10);
        pos1 = (pos1 + score_elf1 as usize + 1) % scores.len();
        pos2 = (pos2 + score_elf2 as usize + 1) % scores.len();
    }
    scores.len() - target.len()
}

pub fn run(s: &str) {
    let content = std::fs::read_to_string(s).unwrap();
    let n = content.strip_suffix("\n").unwrap().parse().unwrap();

    println!("{:?}", part1(n));
    println!("{:?}", part2(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(part1(9), vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
        assert_eq!(part1(5), vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
        assert_eq!(part1(2018), vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2])
    }

    #[test]
    fn example2() {
        assert_eq!(part2(51589), 9);
        assert_eq!(part2(59414), 2018);
    }
}
