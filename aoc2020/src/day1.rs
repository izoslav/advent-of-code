use std::fs;

pub fn run() {
  let input = load("inputs/day1/input.txt");

  println!("=== Day  1 ===");
  println!("First : {}", first(&input));
  println!("Second: {}", second(&input));
}

pub fn first(input: &Vec<u32>) -> u32 {
  let mut answer = 0;

  for x in input {
    if let Some(y) = input.iter().find(|&y| x + y == 2020) {
      answer = x * y;
      break;
    }
  }

  answer
}

pub fn second(input: &Vec<u32>) -> u32{
  let mut answer = 0;

  let mut input = input.clone();
  input.sort();

  for i in 0..input.len() - 2 {
    let mut l = i + 1;
    let mut r = input.len() - 1;

    while l < r {
      let sum = input[l] + input[r] + input[i];

      if sum == 2020 {
        answer = input[l] * input[r] * input[i];
        break;
      }
      else if sum < 2020 {
        l += 1;
      }
      else {
        r -= 1;
      }
    }

    if answer != 0 {
      break;
    }
  }

  answer
}

fn load(filepath: &str) -> Vec<u32> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|word| word.parse().unwrap())
    .collect()
}

#[test]
fn load_test() {
  assert_eq!(load("inputs/example/numbers.txt"), vec![1, 2, 3]);
}

#[test]
fn first_test() {
  let input = load("inputs/day1/example.txt");
  assert_eq!(first(&input), 514579);
}

#[test]
fn second_test() {
  let input = load("inputs/day1/example.txt");
  assert_eq!(second(&input), 241861950);
}