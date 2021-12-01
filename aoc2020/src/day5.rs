use std::fs;

pub fn run() {
  let seats = load("inputs/day5/input.txt");

  println!("=== Day  5 ===");
  println!("Highest seat ID : {}", get_highest_id(&seats));
  println!("Free seat ID    : {}", get_free_seat(&seats));
}

fn get_free_seat(seats: &Vec<(usize, usize)>) -> usize {
  let mut seats = seats
    .iter()
    .map(|(row, column)| calculate_id(*row, *column))
    .collect::<Vec<usize>>();

  seats.sort();

  let mut prev = seats[0];
  for i in 1..seats.len() {
    if seats[i] - prev == 2 {
      return prev + 1;
    }

    prev = seats[i]
  }

  0
}

fn get_highest_id(seats: &Vec<(usize, usize)>) -> usize {
  seats
    .iter()
    .map(|(row, column)| calculate_id(*row, *column))
    .max()
    .unwrap()
}

fn decode_seat(seat: &str) -> (usize, usize) {
  (binary_decode(&seat[..7]), binary_decode(&seat[7..]))
}

fn binary_decode(string: &str) -> usize {
  let binary = string
    .replace("F", "0")
    .replace("L", "0")
    .replace("B", "1")
    .replace("R", "1");

    usize::from_str_radix(&binary, 2).unwrap()
}

fn calculate_id(row: usize, column: usize) -> usize {
  row * 8 + column
}

fn load(filepath: &str) -> Vec<(usize, usize)> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|line| decode_seat(line))
    .collect()
}

#[test]
fn test_decoding() {
  assert_eq!(decode_seat("BFFFBBFRRR"), (70, 7));
  assert_eq!(decode_seat("FFFBBBFRRR"), (14, 7));
  assert_eq!(decode_seat("BBFFBBFRLL"), (102, 4));
}

#[test]
fn test_ids() {
  assert_eq!(calculate_id(70, 7), 567);
  assert_eq!(calculate_id(14, 7), 119);
  assert_eq!(calculate_id(102, 4), 820);
}