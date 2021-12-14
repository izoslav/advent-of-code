use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("res/day14.txt")
        .unwrap()
        .split("\n\n")
        .map(|line| line.to_owned())
        .collect_vec();

    let template = format!("a{}a", input[0])
        .as_bytes()
        .windows(2)
        .map(|bytes| (bytes[0], bytes[1]))
        .counts();

    let rules = input[1]
        .lines()
        .map(|line| {
            let split = line.split(" -> ").collect_vec();
            let pair = split[0].as_bytes();
            let new = split[1].as_bytes()[0];

            ((pair[0], pair[1]), new)
        })
        .collect::<HashMap<_, _>>();

    let (least_common, most_common) = process(template.clone(), &rules, 10);
    println!("Part 1 answer: {}", most_common - least_common);

    let (least_common, most_common) = process(template, &rules, 40);
    println!("Part 2 answer: {}", most_common - least_common);
}

fn process(
    template: HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
    steps: usize,
) -> (usize, usize) {
    let mut counts = HashMap::<u8, usize>::new();

    (0..steps)
        .fold(template, |chain, _| grow(chain, rules))
        .into_iter()
        .for_each(|((l, r), c)| {
            *counts.entry(l).or_insert(0) += c;
            *counts.entry(r).or_insert(0) += c;
        });

    counts
        .into_iter()
        .filter(|&(c, _)| c != b'a')
        .minmax_by(|&(_, a), &(_, b)| a.cmp(&b))
        .into_option()
        .map(|((_, min), (_, max))| (min / 2, max / 2))
        .unwrap()
}

fn grow(
    chain: HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut new = HashMap::<(u8, u8), usize>::new();

    chain.into_iter().for_each(|((l, r), c)| {
        if let Some(&m) = rules.get(&(l, r)) {
            *new.entry((l, m)).or_insert(0) += c;
            *new.entry((m, r)).or_insert(0) += c;
        } else {
            *new.entry((l, r)).or_insert(0) += c;
        }
    });

    new
}
