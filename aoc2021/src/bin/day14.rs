use std::{fs, collections::HashMap};

use itertools::Itertools;

fn main() {
  let input = fs::read_to_string("res/day14.txt")
    .unwrap()
    .split("\n\n")
    .map(|line| line.to_owned())
    .collect_vec();

  let template = input[0].to_string();

  let substitutions = input[1]
    .lines()
    .map(|line| {
      let split = line.split(" -> ").collect_vec();

      let pair = split[0].to_string();
      let substitute = format!("{}{}", &pair[0..1], split[1]);

      (pair, substitute)
    })
    .collect::<HashMap::<String, String>>();

  {
    let mut polymer = template.clone();
    for _ in 0..10 {
      polymer = process(&polymer, &substitutions);
    }

    let letter_counts = count_letters(&polymer);
    
    let (least_common, most_common) = letter_counts
      .into_iter()
      .minmax_by(|&(_, a), &(_, b)| a.cmp(&b))
      .into_option()
      .map(|((_, min), (_, max))| (min, max))
      .unwrap();
    
    println!("Part 1 answer: {}", most_common - least_common);
  }

  {
    let mut polymer = template;
    for _ in 0..40 {
      polymer = process(&polymer, &substitutions);
    }

    let letter_counts = count_letters(&polymer);
    
    let (least_common, most_common) = letter_counts
      .into_iter()
      .minmax_by(|&(_, a), &(_, b)| a.cmp(&b))
      .into_option()
      .map(|((_, min), (_, max))| (min, max))
      .unwrap();
    
    println!("Part 2 answer: {}", most_common - least_common);
  }
}

fn process(template: &str, substitutions: &HashMap::<String, String>) -> String {
  let mut output = String::new();

  for i in 0..template.len() - 1 {
    let pair = &template[i..i+2];
    output.push_str(&substitutions[pair]);
  }

  output.push_str(&template[template.len() - 1..]);
  
  output
}

fn count_letters(polymer: &str) -> HashMap::<char, usize> {
  polymer
    .chars()
    .unique()
    .map(|c| (c, polymer.chars().filter(|ch| *ch == c).count()))
    .collect()
}
