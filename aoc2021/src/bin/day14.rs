use std::{fs, collections::HashMap};

use itertools::Itertools;

fn main() {
  let input = fs::read_to_string("res/day14.txt")
    .unwrap()
    .split("\n\n")
    .map(|line| line.to_owned())
    .collect_vec();

  let template = input[0].to_string();

  let rules = input[1]
    .lines()
    .map(|line| {
      let split = line.split(" -> ").collect_vec();

      let pair = split[0].to_string();
      let substitute = split[1].chars().next().unwrap();

      (pair, substitute)
    })
    .collect::<HashMap::<String, char>>();

  let (least_common, most_common) = process(&template, &rules, 10);
  println!("Part 1 answer: {}", most_common - least_common);

  let (least_common, most_common) = process(&template, &rules, 40);
  println!("Part 2 answer: {}", most_common - least_common);
}

fn process(template: &str, rules: &HashMap::<String, char>, steps: usize) -> (usize, usize) {
  let mut counts = template
    .chars()
    .unique()
    .map(|c| (c, template.chars().filter(|&ch| ch == c).count()))
    .collect();

  for i in 0..template.len() - 1 {
    process_rec(&mut counts, rules, &template[i..i+2], steps);
  }

  counts
    .into_iter()
    .minmax_by(|&(_, a), &(_, b)| a.cmp(&b))
    .into_option()
    .map(|((_, min), (_, max))| (min, max))
    .unwrap()
}

fn process_rec(counts: &mut HashMap::<char, usize>, rules: &HashMap::<String, char>, pair: &str, n: usize) {
  if n == 0 {
    return;
  }

  let letter = rules[pair];
  if !counts.contains_key(&letter) {
    counts.insert(letter, 1);
  } else {
    *counts.get_mut(&letter).unwrap() += 1;
  }

  let left = format!("{}{}", &pair[0..1], letter);
  let right = format!("{}{}", letter, &pair[1..2]);

  process_rec(counts, rules, &left, n - 1);
  process_rec(counts, rules, &right, n - 1);
}
