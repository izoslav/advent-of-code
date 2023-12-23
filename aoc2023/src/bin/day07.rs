use itertools::Itertools;
use std::{collections::HashMap, fs};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn new(symbol: char) -> Self {
        match symbol {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}

impl Type {
    fn new(cards: &str) -> Self {
        let mut counts = HashMap::<char, usize>::new();

        cards
            .chars()
            .for_each(|c| *counts.entry(c).or_insert(0) += 1);

        let mut counts = counts.into_values().collect::<Vec<usize>>();
        counts.sort();
        counts.reverse();

        match counts[0] {
            5 => Self::Five,
            4 => Self::Four,
            3 => match counts[1] {
                2 => Self::FullHouse,
                _ => Self::Three,
            },
            2 => match counts[1] {
                2 => Self::TwoPair,
                _ => Self::OnePair,
            },
            _ => Self::High,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    hand_type: Type,
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn new(input: &str) -> Self {
        let space = input.find(' ').unwrap();

        let cards = input[0..space]
            .chars()
            .map(Card::new)
            .collect::<Vec<Card>>();
        let hand_type = Type::new(&input[0..space]);
        let bid = input[space + 1..input.len()].parse::<usize>().unwrap();

        Hand {
            hand_type,
            cards,
            bid,
        }
    }

    fn promote(&mut self) {
        self.cards = self
            .cards
            .iter()
            .map(|c| match c {
                Card::Jack => Card::Joker,
                _ => *c,
            })
            .collect::<Vec<Card>>();

        let joker_count = self.cards.iter().filter(|c| **c == Card::Joker).count();
        let unique_cards = self.cards.iter().unique().count();

        if joker_count > 0 {
            if joker_count >= 4 {
                self.hand_type = Type::Five;
            } else if joker_count == 3 {
                if unique_cards == 3 {
                    self.hand_type = Type::Four;
                } else {
                    self.hand_type = Type::Five;
                }
            } else if joker_count == 2 {
                if unique_cards == 2 {
                    self.hand_type = Type::Five;
                } else if unique_cards == 3 {
                    self.hand_type = Type::Four;
                } else {
                    self.hand_type = Type::Three;
                }
            } else if joker_count == 1 {
                self.hand_type = match self.hand_type {
                    Type::Four => Type::Five,
                    Type::FullHouse => Type::Four,
                    Type::Three => Type::Four,
                    Type::TwoPair => Type::FullHouse,
                    Type::OnePair => Type::Three,
                    Type::High => Type::OnePair,
                    other => other,
                }
            };
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 != c2 {
                return c1.cmp(c2);
            }
        }

        std::cmp::Ordering::Equal
    }
}

fn main() {
    let hands = fs::read_to_string("inputs/day07.txt")
        .unwrap()
        .lines()
        .map(Hand::new)
        .collect::<Vec<Hand>>();

    let mut sorted_hands = hands.clone();
    sorted_hands.sort();

    let result = sorted_hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>();

    println!("Result part 1: {result}");

    sorted_hands.iter_mut().for_each(|hand| hand.promote());
    sorted_hands.sort();

    let result = sorted_hands
        .iter()
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum::<usize>();

    println!("Result part 2: {result}");
}
