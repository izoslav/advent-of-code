use std::fs;
use itertools::Itertools;


type Field = (String, String);
type Passport = Vec<Field>;

pub fn run() {
  let passports = load("inputs/day4/input.txt");

  println!("=== Day  4 ===");
  println!("Valid passwords : {}", verify_passports(&passports));
}

fn verify_passports(passports: &Vec<Passport>) -> usize {
  let required: Vec<String> = vec![
    "byr".to_string(),
    "iyr".to_string(),
    "eyr".to_string(),
    "hgt".to_string(),
    "hcl".to_string(),
    "ecl".to_string(),
    "pid".to_string()
  ];

  let optional = vec![];

  passports
    .iter()
    .filter(|passport| verify_fields(&passport, &required, &optional))
    .filter(|passport| {
      passport
        .iter()
        .all(|(k, v)| validate_field(k, v))
    })
    .count()
}

fn verify_fields(
  passport: &Passport,
  required: &Vec<String>,
  _optional: &Vec<String>
) -> bool {
  let fields: Vec<&String> = passport
    .iter()
    .map(|(k, _)| k)
    .collect();

  required
    .iter()
    .all(|req| fields.contains(&req))
}

fn validate_field(
  key: &String,
  value: &String
) -> bool {
  match &key[..] {
    "byr" => {
      let year = value.parse::<usize>().unwrap();
      between(year, 1920, 2002)
    }
    "iyr" => {
      let year = value.parse::<usize>().unwrap();
      between(year, 2010, 2020)
    }
    "eyr" => {
      let year = value.parse::<usize>().unwrap();
      between(year, 2020, 2030)
    }
    "hgt" => {
      if let Ok(height) = value[..value.len() - 2].parse::<usize>() {
        match &value[value.len() - 2..] {
          "cm" => { between(height, 150, 193) }
          "in" => { between(height, 59, 76) }
          _ => false
        }
      } else {
        false
      }
    }
    "hcl" => {
      if value.as_bytes()[0] as char == '#' {
        value[1..]
          .chars()
          .all(|c| "0123456789abcdef".contains(c))
      }
      else {
        false
      }
    }
    "ecl" => {
      vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|color| color == value)
    }
    "pid" => {
      value.len() == 9
    }
    "cid" => { true }
    _ => false
  }
}

fn between(value: usize, min: usize, max: usize) -> bool {
  value >= min && value <= max
}

fn load(filepath: &str) -> Vec<Passport> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split("\n\n")
    .map(|entry| {
      entry
        .split_whitespace()
        .map(|field| {
          let (k, v) = field
            .split(':')
            .collect_tuple()
            .unwrap();

          (String::from(k), String::from(v))
        })
        .collect()
    })
    .collect()
}

#[test]
fn test_verify_okay() {
  let field = (String::from("foo"), String::from("abc"));
  let passport = vec![field];
  let required = vec![String::from("foo")];
  let optional = vec![];

  assert_eq!(verify_fields(&passport, &required, &optional), true);
}

#[test]
fn test_verify_fail() {
  let field = (String::from("foo"), String::from("abc"));
  let passport = vec![field];
  let required = vec![String::from("bar")];
  let optional = vec![];

  assert_eq!(verify_fields(&passport, &required, &optional), false);
}

#[test]
fn test_examples() {
  let passports = load("inputs/day4/example.txt");

  assert_eq!(verify_passports(&passports), 2)
}