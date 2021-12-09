use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

fn main() {
    let input: Vec<(Vec<String>, Vec<String>)> = fs::read_to_string("res/day08.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" | ").collect();

            let patterns: Vec<String> = split[0]
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .collect();

            let input: Vec<String> = split[1]
                .split(' ')
                .map(|s| s.chars().sorted().collect::<String>())
                .collect();

            (patterns, input)
        })
        .collect();

    let mut unique_segments = 0;
    let mut output_value = 0;

    for (patterns, input) in &input {
        let decoded_patterns = decode_patterns(patterns);
        let decoded_input = decode_input(&decoded_patterns, input);

        unique_segments += decoded_input
            .iter()
            .filter(|&i| [1, 4, 7, 8].contains(i))
            .count();

        let pattern_to_num = decode_segments(patterns);

        output_value += input
            .iter()
            .map(|i| char::from_digit(pattern_to_num[i], 10).unwrap())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
    }

    println!("Part 1 answer: {}", unique_segments);
    println!("Part 2 answer: {}", output_value);
}

fn decode_patterns(input: &[String]) -> HashMap<String, u8> {
    let mut decoded_patterns = HashMap::<String, u8>::new();

    input.iter().for_each(|i| match i.len() {
        2 => {
            decoded_patterns.insert(i.to_string(), 1);
        }
        3 => {
            decoded_patterns.insert(i.to_string(), 7);
        }
        4 => {
            decoded_patterns.insert(i.to_string(), 4);
        }
        7 => {
            decoded_patterns.insert(i.to_string(), 8);
        }
        _ => (),
    });

    decoded_patterns
}

fn decode_segments(input: &[String]) -> HashMap<String, u32> {
    let five_segments = input
        .iter()
        .filter(|&s| s.len() == 5)
        .cloned()
        .collect::<Vec<String>>();

    let six_segments = input
        .iter()
        .filter(|&s| s.len() == 6)
        .cloned()
        .collect::<Vec<String>>();

    let mut num_to_pattern = vec![""; 10];

    input.iter().for_each(|i| match i.len() {
        2 => {
            num_to_pattern[1] = i;
        }
        3 => {
            num_to_pattern[7] = i;
        }
        4 => {
            num_to_pattern[4] = i;
        }
        7 => {
            num_to_pattern[8] = i;
        }
        _ => (),
    });

    num_to_pattern[3] = five_segments
        .iter()
        .find(|&s| intersect_len(s, num_to_pattern[1]) == 2)
        .unwrap();

    num_to_pattern[9] = six_segments
        .iter()
        .find(|&s| intersect_len(s, num_to_pattern[3]) == 5)
        .unwrap();

    num_to_pattern[2] = five_segments
        .iter()
        .find(|&s| intersect_len(s, num_to_pattern[9]) == 4)
        .unwrap();

    num_to_pattern[5] = five_segments
        .iter()
        .find(|&s| {
            intersect_len(s, num_to_pattern[9]) == 5 && intersect_len(s, num_to_pattern[1]) == 1
        })
        .unwrap();

    num_to_pattern[6] = six_segments
        .iter()
        .find(|&s| intersect_len(s, num_to_pattern[5]) == 5 && s != num_to_pattern[9])
        .unwrap();

    num_to_pattern[0] = six_segments
        .iter()
        .find(|&s| s != num_to_pattern[6] && s != num_to_pattern[9])
        .unwrap();

    num_to_pattern
        .iter()
        .enumerate()
        .map(|(i, &pattern)| (pattern.to_string(), i as u32))
        .collect()
}

fn decode_input(patterns: &HashMap<String, u8>, input: &[String]) -> Vec<u8> {
    input
        .iter()
        .map(|i| *patterns.get(i).unwrap_or(&0))
        .collect::<Vec<u8>>()
}

fn intersect(a: &str, b: &str) -> String {
    a.chars().filter(|&c| b.contains(c)).collect::<String>()
}

fn intersect_len(a: &str, b: &str) -> usize {
    intersect(a, b).len()
}
