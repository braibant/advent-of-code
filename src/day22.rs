use std::collections::HashSet;

fn parse(s: &str) -> Vec<usize> {
    let lines: Vec<_> = s.split("\n").collect();
    lines[1..]
        .iter()
        .map(|s| {
            let u: usize = s.parse().unwrap();
            u
        })
        .collect()
}

fn score(deck: &Vec<usize>) -> u64 {
    let mut deck: Vec<usize> = deck.clone();
    deck.reverse();

    let mut score: u64 = 0;
    for (idx, &card) in deck.iter().enumerate() {
        score += (card as u64) * ((idx + 1) as u64)
    }

    score
}

mod part1 {
    use super::*;
    fn play_round(p1: &mut Vec<usize>, p2: &mut Vec<usize>) {
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

    pub fn play(player1_deck: &Vec<usize>, player2_deck: &Vec<usize>) -> u64 {
        let mut player1_deck = player1_deck.clone();
        let mut player2_deck = player2_deck.clone();

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

        return winner_score;
    }
}

mod part2 {
    use super::*;
    #[derive(Debug)]
    pub enum Winner {
        Player1,
        Player2,
    }

    fn play_rec(player1_deck: &mut Vec<usize>, player2_deck: &mut Vec<usize>) -> Winner {
        let mut previous_positions: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

        while player1_deck.len() != 0 && player2_deck.len() != 0 {
            let key = (player1_deck.clone(), player2_deck.clone());
            if previous_positions.contains(&key) {
                return (Winner::Player1);
            } else {
                previous_positions.insert(key);
            }

            let c1 = player1_deck.remove(0);
            let c2 = player2_deck.remove(0);

            let winner = {
                if c1 <= player1_deck.len() && c2 <= player2_deck.len() {
                    let mut p1: Vec<_> = player1_deck[0..c1].to_vec();
                    let mut p2: Vec<_> = player2_deck[0..c2].to_vec();
                    play_rec(&mut p1, &mut p2)
                } else {
                    // One of the player does not have enough card, the winner is the player with the highest-value card
                    if c1 > c2 {
                        Winner::Player1
                    } else {
                        Winner::Player2
                    }
                }
            };

            match winner {
                Winner::Player1 => {
                    player1_deck.push(c1);
                    player1_deck.push(c2);
                }
                Winner::Player2 => {
                    player2_deck.push(c2);
                    player2_deck.push(c1);
                }
            }
        }

        if player1_deck.len() == 0 {
            return Winner::Player2;
        } else {
            return Winner::Player1;
        }
    }

    pub fn play(player1_deck: &Vec<usize>, player2_deck: &Vec<usize>) -> (Winner, u64) {
        let mut player1_deck = player1_deck.clone();
        let mut player2_deck = player2_deck.clone();

        let winner = play_rec(&mut player1_deck, &mut player2_deck);
        let score = match winner {
            Winner::Player1 => score(&player1_deck),
            Winner::Player2 => score(&player2_deck),
        };
        return (winner, score);
    }
}

pub fn run(filename: String) {
    let contents = std::fs::read_to_string(filename).unwrap();

    let players: Vec<_> = contents.split("\n\n").collect();

    let player1_deck: Vec<_> = parse(players[0]);
    let player2_deck: Vec<_> = parse(players[1]);

    println!("{}", part1::play(&player1_deck, &player2_deck));
    println!("{:?}", part2::play(&player1_deck, &player2_deck));
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
