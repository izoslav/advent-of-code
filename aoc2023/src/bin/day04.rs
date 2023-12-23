use std::{collections::HashSet, fs};

#[derive(Debug)]
struct GameCard {
    _id: u32,
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl GameCard {
    fn new(input: &str) -> Self {
        let colon = input.find(':').unwrap();
        let bar = input.find('|').unwrap();

        let id = input[5..colon].trim().parse::<u32>().unwrap();
        let numbers = input[colon + 2..bar - 1]
            .trim()
            .replace("  ", " ")
            .split(' ')
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect();
        let winning_numbers = input[bar + 2..input.len()]
            .trim()
            .replace("  ", " ")
            .split(' ')
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect();

        Self {
            _id: id,
            numbers,
            winning_numbers,
        }
    }

    fn calculate_matches(&self) -> u32 {
        self.numbers
            .intersection(&self.winning_numbers)
            .collect::<Vec<&u32>>()
            .len() as u32
    }

    fn calculate_points(&self) -> u32 {
        let n = self
            .numbers
            .intersection(&self.winning_numbers)
            .collect::<Vec<&u32>>()
            .len();

        if n > 0 {
            2u32.pow(n as u32 - 1)
        } else {
            0
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").unwrap();

    let result = input
        .lines()
        .map(GameCard::new)
        .map(|gc| gc.calculate_points())
        .sum::<u32>();

    println!("Result part 1: {result}");

    let cards = input
        .lines()
        .map(GameCard::new)
        .collect::<Vec<GameCard>>();

    let mut copies = Vec::new();
    copies.resize(cards.len(), 1);

    let result = cards
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let n = c.calculate_matches();
            for j in i + 1..(i + 1 + n as usize).min(copies.len()) {
                copies[j] += copies[i];
            }
            copies[i]
        })
        .sum::<u32>();

    println!("Result part 2: {result}");
}
