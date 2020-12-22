use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;

type Deck = VecDeque<usize>;

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

fn play_recursive_game(mut decks: (Deck, Deck)) -> (Winner, Deck) {
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();

    while !decks.0.is_empty() && !decks.1.is_empty() {
        if !seen.insert((decks.0.clone(), decks.1.clone())) {
            // We've already seen these decks in a previous round - player 1
            // wins
            return (Winner::Player1, decks.0)
        }
        let top_0 = decks.0.pop_front().unwrap();
        let top_1 = decks.1.pop_front().unwrap();
        if decks.0.len() >= top_0 && decks.1.len() >= top_1 {
            // Determine the winner by playing a sub-game
            match play_recursive_game((
                decks.0.iter().take(top_0).cloned().collect(),
                decks.1.iter().take(top_1).cloned().collect(),
            )).0 {
                Winner::Player1 => {
                    decks.0.push_back(top_0);
                    decks.0.push_back(top_1);
                }
                Winner::Player2 => {
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
    winner
        .iter()
        .enumerate()
        .map(|(i, card)| *card * (winner.len() - i))
        .sum()
}

fn part_two(decks: &(Deck, Deck)) -> usize {
    let decks = decks.clone();
    let winner = play_recursive_game(decks).1;
    winner
        .iter()
        .enumerate()
        .map(|(i, card)| *card * (winner.len() - i))
        .sum()
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
