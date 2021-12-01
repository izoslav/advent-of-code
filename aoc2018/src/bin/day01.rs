use std::collections::BTreeSet;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("res/day01_1.txt").unwrap();

    let frequencies: Vec<i32> = input
        .lines()
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let resulting_frequency: i32 = frequencies.iter().sum();
    println!(
        "Part 1 answer: Resulting frequency = {}",
        resulting_frequency
    );

    let mut partial_frequencies = BTreeSet::new();
    let mut current_frequency = 0;

    for f in std::iter::repeat(frequencies).flat_map(|x| x.into_iter()) {
        current_frequency += f;

        if !partial_frequencies.contains(&current_frequency) {
            partial_frequencies.insert(current_frequency);
        } else {
            break;
        }
    }

    println!(
        "Part 2 answer: first duplicate frequency = {}",
        current_frequency
    );
}
