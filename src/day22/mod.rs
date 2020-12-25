use itertools::Itertools;
use std::collections::{hash_map::DefaultHasher, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[exec_time]
fn day22_part01<'a>(string: &'a str) {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();

    let (deck1, deck2) = string
        .strip_prefix("Player 1:")
        .unwrap()
        .trim_start()
        .splitn(2, "Player 2:")
        .next_tuple()
        .unwrap();

    for card in deck1.lines() {
        if card.is_empty() {
            continue;
        }
        player1.push_back(card.parse::<usize>().unwrap());
    }

    for card in deck2.lines() {
        if card.is_empty() {
            continue;
        }
        player2.push_back(card.parse::<usize>().unwrap());
    }

    while !(player1.is_empty() || player2.is_empty()) {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if card1 > card2 {
            // Player 1 wins.
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            // Player 2 wins.
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    let mut winning_player = if player1.is_empty() { player2 } else { player1 };

    let mut total = 0;

    for score in 0..winning_player.len() {
        let score = score + 1;

        total += score * winning_player.pop_back().unwrap();
    }

    red_ln!("Day 22, part 01: Winning score {}", total);
}

fn does_player1_win_recursive_game(
    player1: &mut VecDeque<usize>,
    player2: &mut VecDeque<usize>,
) -> bool {
    let mut been_here_before = HashSet::new();

    while !(player1.is_empty() || player2.is_empty()) {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        let player1_wins = if (card1 <= player1.len()) && (card2 <= player2.len()) {
            let mut clone1 = VecDeque::new();
            for item in player1.iter().take(card1) {
                clone1.push_back(*item);
            }

            let mut clone2 = VecDeque::new();
            for item in player2.iter().take(card2) {
                clone2.push_back(*item);
            }

            does_player1_win_recursive_game(&mut clone1, &mut clone2)
        } else {
            card1 > card2
        };

        if player1_wins {
            // Player 1 wins.
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            // Player 2 wins.
            player2.push_back(card2);
            player2.push_back(card1);
        }

        let mut hasher = DefaultHasher::new();
        player1.hash(&mut hasher);
        player2.hash(&mut hasher);
        let hash = hasher.finish();

        // Player 1 wins (we're in infinite recursion otherwise...!)
        if !been_here_before.insert(hash) {
            return true;
        }
    }

    !player1.is_empty()
}

#[exec_time]
fn day22_part02(string: &str) {
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();

    let (deck1, deck2) = string
        .strip_prefix("Player 1:")
        .unwrap()
        .trim_start()
        .splitn(2, "Player 2:")
        .next_tuple()
        .unwrap();

    for card in deck1.lines() {
        if card.is_empty() {
            continue;
        }
        player1.push_back(card.parse::<usize>().unwrap());
    }

    for card in deck2.lines() {
        if card.is_empty() {
            continue;
        }
        player2.push_back(card.parse::<usize>().unwrap());
    }

    let mut winning_player = if does_player1_win_recursive_game(&mut player1, &mut player2) {
        player1
    } else {
        player2
    };

    let mut total = 0;

    for score in 0..winning_player.len() {
        let score = score + 1;

        total += score * winning_player.pop_back().unwrap();
    }

    green_ln!("Day 22, part 02: Winning score {}", total);
}

pub fn run() {
    let input = include_bytes!("input");
    let string = String::from_utf8(input.to_vec()).unwrap();

    day22_part01(&string);
    day22_part02(&string);
}
