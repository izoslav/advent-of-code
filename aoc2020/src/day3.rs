use std::fs;

pub fn run() {
  let forest = load("inputs/day3/input.txt");

  println!("=== Day  3 ===");
  println!("First : {}", check_slope(&forest, 3, 1));
  println!("Second: {}", check_multiple_slopes(&forest, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]));
}

fn check_slope(forest: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
  let mut x = 0;
  let mut y = 0;
  let mut hits = 0;

  while y < forest.len() {
    if forest[y][x] == '#' {
      hits += 1
    }

    x = (x + right) % forest[0].len();
    y += down;
  }

  hits
}

fn check_multiple_slopes(forest: &Vec<Vec<char>>, slopes: Vec<(usize, usize)>) -> usize {
  slopes.iter()
    .map(|(right, down)| {
      check_slope(&forest, *right, *down)
    })
    .product()
}

fn load(filepath: &str) -> Vec<Vec<char>> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|line| {
      line.chars().collect()
    })
    .collect()
}

#[test]
fn first_test() {
  let forest = load("inputs/day3/example.txt");

  assert_eq!(check_slope(&forest, 3, 1), 7);
  assert_eq!(check_multiple_slopes(&forest, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]), 336);
}