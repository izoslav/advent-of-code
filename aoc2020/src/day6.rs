use std::fs;
use itertools::Itertools;

pub fn run() {
  let groups = load("inputs/day6/input.txt");

  let unique: usize = groups
    .iter()
    .map(|g| count_unique(g))
    .collect::<Vec<usize>>()
    .iter()
    .sum();

  let common: usize = groups
    .iter()
    .map(|g| count_common(g))
    .collect::<Vec<usize>>()
    .iter()
    .sum();

  println!("=== Day  6 ===");
  println!("Sum of unique answers : {}", unique);
  println!("Sum of common answers : {}", common);
}

fn count_unique(group: &Vec<String>) -> usize {
  group
    .join("")
    .chars()
    .unique()
    .collect::<String>()
    .len()
}

fn count_common(group: &Vec<String>) -> usize {
  group[1..]
    .iter()
    .fold(String::from(&group[0]), |acc, answers| {
      acc
        .chars()
        .filter(|c| {
          answers.contains(*c)
        })
        .collect()
    })
    .len()
}

fn load(filepath: &str) -> Vec<Vec<String>> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split("\n\n")
    .map(|entry| {
      entry
        .split_whitespace()
        .map(|entry| entry.to_string())
        .collect()
    })
    .collect()
}

#[test]
fn test_count_unique() {
  let group = vec!["ab".to_string(), "ac".to_string(), "abc".to_string()];
  assert_eq!(count_unique(&group), 3);
}

#[test]
fn test_count_common() {
  let group = vec!["ab".to_string(), "ac".to_string(), "abc".to_string()];
  assert_eq!(count_common(&group), 1);
}

#[test]
fn test_example() {
  let groups = load("inputs/day6/example.txt");

  let unique_answers: Vec<usize> = groups
    .iter()
    .map(|group| count_unique(group))
    .collect();

  assert_eq!(unique_answers, vec![3, 3, 3, 1, 1]);
  assert_eq!(unique_answers.iter().sum::<usize>(), 11);

  let common_answers: Vec<usize> = groups
    .iter()
    .map(|group| count_common(group))
    .collect();

  assert_eq!(common_answers, vec![3, 0, 1, 1, 1]);
  assert_eq!(common_answers.iter().sum::<usize>(), 6);
}