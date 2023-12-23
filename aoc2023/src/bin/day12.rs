use rayon::prelude::*;
use std::{collections::HashMap, fs};

fn count_arrangements(input: &str, groups: &[usize]) -> usize {
    let mut memo = HashMap::new();

    arrangements(input, groups, &mut memo)
}

fn arrangements(input: &str, groups: &[usize], memo: &mut HashMap<String, usize>) -> usize {
    if memo.contains_key(input) {
        return *memo.get(input).unwrap();
    }

    if !validate(input, groups) {
        memo.insert(input.to_string(), 0);
        return 0;
    }

    if !input.contains('?') {
        memo.insert(input.to_string(), 1);
        return 1;
    }

    let result = arrangements(&input.replacen('?', ".", 1), groups, memo)
        + arrangements(&input.replacen('?', "#", 1), groups, memo);

    memo.insert(input.to_string(), result);

    result
}

fn validate(input: &str, groups: &[usize]) -> bool {
    let input = input
        .replace('.', " ")
        .trim()
        .split_whitespace()
        .map(|group| format!("{group}"))
        .collect::<Vec<String>>();

    input
        .iter()
        .enumerate()
        .all(|(idx, group)| group.len() == groups[idx])
}

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();

    let result = input
        .par_lines()
        .map(|line| {
            let space = line.find(' ').unwrap();
            let input = &line[0..space];
            let groups = &line[space + 1..line.len()]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            count_arrangements(input, groups)
        })
        .sum::<usize>();

    println!("Result part 1: {result}");

    let result = input
        .par_lines()
        .map(|line| {
            let space = line.find(' ').unwrap();
            let input = &line[0..space];
            let input = format!("{input}?{input}?{input}?{input}?{input}");

            let groups = line[space + 1..line.len()]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let groups = vec![
                groups.clone(),
                groups.clone(),
                groups.clone(),
                groups.clone(),
                groups.clone(),
            ]
            .concat();

            count_arrangements(&input, &groups)
        })
        .sum::<usize>();

    println!("Result part 2: {result}");
}
