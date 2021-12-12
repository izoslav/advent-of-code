use std::fs;

use itertools::Itertools;

enum Status {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn main() {
    let input = fs::read_to_string("res/day10.txt")
        .unwrap()
        .lines()
        .map(|line| diagnose_line(line))
        .collect::<Vec<Status>>();

    let errors = input
        .iter()
        .filter(|&status| matches!(status, Status::Corrupted(_)))
        .map(|status| {
            if let Status::Corrupted(c) = status {
                *c
            } else {
                panic!()
            }
        })
        .collect::<Vec<char>>();

    let normal_score = errors.iter().filter(|&c| *c == ')').count() * 3;
    let square_score = errors.iter().filter(|&c| *c == ']').count() * 57;
    let curly_score = errors.iter().filter(|&c| *c == '}').count() * 1197;
    let triangular_score = errors.iter().filter(|&c| *c == '>').count() * 25137;

    let error_score = normal_score + square_score + curly_score + triangular_score;

    println!("Part 1 answer: {}", error_score);

    let missing_scores = input
        .iter()
        .filter(|&status| matches!(status, Status::Incomplete(_)))
        .map(|status| {
            if let Status::Incomplete(v) = status {
                v
            } else {
                panic!()
            }
        })
        .map(|v| calculate_completion_score(v))
        .sorted()
        .collect::<Vec<usize>>();

    let missing_score = missing_scores[missing_scores.len() / 2];
    println!("Part 2 answer: {}", missing_score);
}

fn diagnose_line(input: &str) -> Status {
    let mut stack = Vec::<char>::new();

    for closing in input.chars() {
        match closing {
            '(' | '<' | '[' | '{' => stack.push(closing),
            _ => {
                if let Some(last) = stack.pop() {
                    if last != match_opening(closing) {
                        return Status::Corrupted(closing);
                    }
                }
            }
        }
    }

    let closing = stack.iter().rev().map(|&c| match_closing(c)).collect();

    Status::Incomplete(closing)
}

fn match_opening(closing: char) -> char {
    match closing {
        ')' => '(',
        '>' => '<',
        ']' => '[',
        '}' => '{',
        _ => panic!(),
    }
}

fn match_closing(opening: char) -> char {
    match opening {
        '(' => ')',
        '<' => '>',
        '[' => ']',
        '{' => '}',
        _ => panic!(),
    }
}

fn calculate_completion_score(input: &[char]) -> usize {
    input.iter().fold(0, |acc, &c| {
        5 * acc
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            }
    })
}
