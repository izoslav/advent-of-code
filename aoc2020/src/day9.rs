use std::fs;

pub fn run() {
  let input = load("inputs/day9/input.txt");
  let exploit = find_exploit(&input, 25);
  let sum = find_contiguous_sum(&input, exploit);

  println!("=== Day  9 ===");
  println!("First invalid: {}", exploit);
  println!("Exploit sum: {}", sum);
}

fn find_exploit(input: &Vec<u64>, preamble_size: usize) -> u64 {
  let mut buffer: Vec<u64> = input.iter().take(preamble_size).cloned().collect();
  let mut pos = 0;

  for value in &input[preamble_size..input.len()] {
    let ok = buffer
      .iter()
      .map(|x| {
        match &buffer.iter().find(|&y| x != y && x + y == *value) {
          Some(_) => { true }
          _ => { false }
        }
      })
      .any(|x| x);

    if !ok {
      return *value;
    }
    
    buffer[pos] = *value;
    pos = (pos + 1) % preamble_size;
  }

  0
}

fn find_contiguous_sum(input: &Vec<u64>, value: u64) -> u64 {
  for pos in 0..input.len() {
    for i in 1..input.len() - pos {
      let sum: u64 = input[pos..pos + i].iter().sum();

      if sum == value {
        let min = input[pos..pos + i].iter().min().unwrap();
        let max = input[pos..pos + i].iter().max().unwrap();
        return min + max;
      }
      else if sum > value {
        break;
      }
    }
  }

  0
}

fn load(filepath: &str) -> Vec<u64> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|number| number.parse::<u64>().unwrap())
    .collect()
}

#[test]
fn test_load() {
  let expected = vec![
    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576
  ];

  assert_eq!(expected, load("inputs/day9/example.txt"));
}

#[test]
fn test_example() {
  let input = load("inputs/day9/example.txt");
  let expected_exploit = 127;
  let expected_sum = 62;

  assert_eq!(expected_exploit, find_exploit(&input, 5));
  assert_eq!(expected_sum, find_contiguous_sum(&input, expected_exploit));
}