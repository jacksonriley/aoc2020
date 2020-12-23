use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Instant;

type Deck = VecDeque<u8>;

enum Winner {
    Player1,
    Player2,
}

fn play_simple_round(decks: &mut (Deck, Deck)) {
    let top_0 = decks.0.pop_front().unwrap();
    let top_1 = decks.1.pop_front().unwrap();
    match top_0.cmp(&top_1) {
        Ordering::Greater => {
            decks.0.push_back(top_0);
            decks.0.push_back(top_1);
        }
        Ordering::Less => {
            decks.1.push_back(top_1);
            decks.1.push_back(top_0);
        }
        Ordering::Equal => panic!("Ties should not occur"),
    }
}

fn play_recursive_game(mut decks: (Deck, Deck), top_game: bool) -> (Winner, Deck) {
    // Short circuit sub-game if player 1 has the largest card - as in this
    // event, player 1 will always win, and that's all that we care about.
    if !top_game {
        let max_1 = decks.0.iter().max().unwrap();
        if decks.1.iter().all(|card| card < max_1) {
            return (Winner::Player1, decks.0);
        }
    }

    let mut seen: HashSet<u64> = HashSet::new();

    while !decks.0.is_empty() && !decks.1.is_empty() {
        if !seen.insert(hash_decks(&decks)) {
            // We've already seen these decks in a previous round - player 1
            // wins
            return (Winner::Player1, decks.0);
        }
        let top_0 = decks.0.pop_front().unwrap();
        let top_1 = decks.1.pop_front().unwrap();
        if decks.0.len() >= top_0 as usize && decks.1.len() >= top_1 as usize {
            // Determine the winner by playing a sub-game
            match play_recursive_game(
                (
                    decks.0.iter().take(top_0 as usize).copied().collect(),
                    decks.1.iter().take(top_1 as usize).copied().collect(),
                ),
                false,
            ) {
                (Winner::Player1, _) => {
                    decks.0.push_back(top_0);
                    decks.0.push_back(top_1);
                }
                (Winner::Player2, _) => {
                    decks.1.push_back(top_1);
                    decks.1.push_back(top_0);
                }
            }
        } else {
            // The higher value card wins
            match top_0.cmp(&top_1) {
                Ordering::Greater => {
                    decks.0.push_back(top_0);
                    decks.0.push_back(top_1);
                }
                Ordering::Less => {
                    decks.1.push_back(top_1);
                    decks.1.push_back(top_0);
                }
                Ordering::Equal => panic!("Ties should not occur"),
            }
        }
    }

    if decks.0.is_empty() {
        (Winner::Player2, decks.1)
    } else {
        (Winner::Player1, decks.0)
    }
}

fn hash_decks(decks: &(Deck, Deck)) -> u64 {
    let mut hasher = DefaultHasher::new();
    decks.0.hash(&mut hasher);
    decks.1.hash(&mut hasher);
    hasher.finish()
}

fn calculate_score(deck: &Deck) -> usize {
    deck.iter()
        .enumerate()
        .map(|(i, card)| *card as usize * (deck.len() - i))
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/22")?;
    let decks = parse_input(&input);
    println!("Part 1: {}", part_one(&decks));
    println!("Part 2: {}", part_two(&decks));
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let mut decks = input.split("\n\n");
    (
        decks
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|l| l.parse().unwrap())
            .collect(),
        decks
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|l| l.parse().unwrap())
            .collect(),
    )
}

fn part_one(decks: &(Deck, Deck)) -> usize {
    let mut decks = decks.clone();
    while !decks.0.is_empty() && !decks.1.is_empty() {
        play_simple_round(&mut decks);
    }
    let mut winner = &decks.0;
    if decks.0.is_empty() {
        winner = &decks.1;
    }
    calculate_score(winner)
}

fn part_two(decks: &(Deck, Deck)) -> usize {
    let decks = decks.clone();
    let winner = play_recursive_game(decks, true).1;
    calculate_score(&winner)
}

#[test]
fn test_examples() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    let decks = parse_input(input);
    assert_eq!(part_one(&decks), 306);
    assert_eq!(part_two(&decks), 291);
}
