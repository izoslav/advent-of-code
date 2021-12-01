use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Rule {
  a: usize,
  b: usize,
  c: char,
  password: String
}

pub fn run() {
  let rules = load("inputs/day2/input.txt");

  println!("=== Day  2 ===");
  println!("First : {}", first(&rules));
  println!("Second: {}", second(&rules));
}

fn first(rules: &Vec<Rule>) -> u32 {
  rules
    .iter()
    .map(|rule| {
      match rule {
        Rule { a: min, b: max, c: letter, password } => {
          let count = &password.matches(*letter).count();
          if count >= min && count <= max { 1 } else { 0 }
        }
      }
    })
    .sum()
}

fn second(rules: &Vec<Rule>) -> u32 {
  rules
    .iter()
    .map(|rule| {
      match rule {
        Rule { a: i1, b: i2, c: letter, password } => {
          let p1 = password.as_bytes()[i1 - 1] as char;
          let p2 = password.as_bytes()[i2 - 1] as char;

          if (p1 == *letter) ^ (p2 == *letter) { 1 } else { 0 }
        }
      }
    })
    .sum()
}

fn parse_rule(rule: &str) -> Rule {
  let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
  let cap = re.captures(rule).unwrap();

  Rule {
    a: cap[1].parse::<usize>().unwrap(),
    b: cap[2].parse::<usize>().unwrap(),
    c: cap[3].as_bytes()[0] as char,
    password: String::from(&cap[4])
  }
}

fn load(filepath: &str) -> Vec<Rule> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split('\n')
    .map(|line| parse_rule(line))
    .collect()
}

#[test]
fn first_test() {
  let rules = load("inputs/day2/example.txt");

  assert_eq!(first(&rules), 2);
}

#[test]
fn second_test() {
  let rules = load("inputs/day2/example.txt");

  assert_eq!(second(&rules), 1);
}
