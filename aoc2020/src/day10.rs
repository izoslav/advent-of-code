use std::fs;

use itertools::Itertools;

pub fn run() {
  let input = load("inputs/day10/input.txt");
  let differences = count_joltage_differences(&input);

  println!("=== Day 10 ===");
  println!("1-jolt diffs * 3-jolt diffs: {}", differences.0 * differences.2);
  println!("Possible combinations: {}", count_combinations(&input))
}


fn load(filepath: &str) -> Vec<u32> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  let mut output = contents
    .trim()
    .split_whitespace()
    .map(|number| number.parse::<u32>().unwrap())
    .sorted()
    .collect::<Vec<u32>>();

  output.insert(0, 0);
  output.push(output[output.len() - 1] + 3);

  output
}

fn count_joltage_differences(input: &Vec<u32>) -> (u32, u32, u32) {
  let mut diff1 = 0;
  let mut diff2 = 0;
  let mut diff3 = 0;

  for window in input.windows(2) {
    let diff: i32 = window[1] as i32 - window[0] as i32;
    match diff {
      1 => { diff1 += 1 }
      2 => { diff2 += 1 }
      3 => { diff3 += 1 }
      _ => ()
    }
  }

  (diff1, diff2, diff3)
}

fn count_combinations(input: &Vec<u32>) -> u64 {
  let mut slices = vec![];
  let mut current_slice = vec![];

  for window in input.windows(2) {
    match window[1] - window[0] {
      1 => current_slice.push(window[0]),
      3 => {
        current_slice.push(window[0]);
        slices.push(current_slice);
        current_slice = vec![];
      }
      _ => (),
    }
  }

  slices
    .iter()
    .map(|slice| match slice.len() {
      1 => 1,
      2 => 1,
      3 => 2,
      4 => 4,
      5 => 7,
      _ => panic!("unexpected slice of size N > 5 consecutive 1-diff elements"),
    })
    .product()
}

#[test]
fn test_small_example() {
  let input = load("inputs/day10/example_small.txt");
  println!("{:?}", &input);

  let expected_d1 = 7;
  let expected_d3 = 5;
  let differences = count_joltage_differences(&input);

  assert_eq!(expected_d1, differences.0);
  assert_eq!(expected_d3, differences.2);

  let expected_combinations = 8;
  let combinations = count_combinations(&input);

  assert_eq!(expected_combinations, combinations);
}

#[test]
fn test_big_example() {
  let input = load("inputs/day10/example_big.txt");
  println!("{:?}", &input);

  let expected_d1 = 22;
  let expected_d3 = 10;
  let differences = count_joltage_differences(&input);

  assert_eq!(expected_d1, differences.0);
  assert_eq!(expected_d3, differences.2);

  let expected_combinations = 19208;
  let combinations = count_combinations(&input);

  assert_eq!(expected_combinations, combinations);
}