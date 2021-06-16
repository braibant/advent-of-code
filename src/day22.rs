fn parse(s: &str) -> Vec<u32> {
    let lines: Vec<_> = s.split("\n").collect();
    lines[1..]
        .iter()
        .map(|s| {
            let u: u32 = s.parse().unwrap();
            u
        })
        .collect()
}

fn play_round(p1: &mut Vec<u32>, p2: &mut Vec<u32>) {
    if p1.len() == 0 || p2.len() == 0 {
        panic!("Invalid arguments")
    };

    let c1 = p1.remove(0);
    let c2 = p2.remove(0);

    if c1 > c2 {
        p1.push(c1);
        p1.push(c2);
    } else {
        // c2 > c1
        p2.push(c2);
        p2.push(c1);
    }
}

fn score(deck: &Vec<u32>) -> u64 {
    let mut deck: Vec<u32> = deck.clone();
    deck.reverse();

    let mut score: u64 = 0;
    for (idx, &card) in deck.iter().enumerate() {
        score += (card as u64) * ((idx + 1) as u64)
    }

    score
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let players: Vec<_> = contents.split("\n\n").collect();

    let mut player1_deck: Vec<_> = parse(players[0]);
    let mut player2_deck: Vec<_> = parse(players[1]);

    while player1_deck.len() != 0 && player2_deck.len() != 0 {
        play_round(&mut player1_deck, &mut player2_deck)
    }

    let winner_score = {
        if player1_deck.len() == 0 {
            score(&player2_deck)
        } else {
            score(&player1_deck)
        }
    };

    println!("{}", winner_score);
}

#[cfg(test)]
mod test {

    use super::score;
    #[test]
    fn test_score() {
        let deck = vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1];

        assert_eq!(score(&deck), 306);
    }
}
