use std::fs;

pub fn run() {
  let instructions = load("inputs/day8/input.txt");

  println!("=== Day  8 ===");
  println!("Acc before looping: {}", execute(&instructions).0);
  println!("Acc after fixing: {}", fix(&instructions));
}

fn execute(instructions: &Vec<(String, i32)>) -> (i32, usize) {
  let mut visited = vec![false; instructions.len()];
  let mut acc = 0i32;
  let mut ic = 0;

  while ic < instructions.len() && !visited[ic] {
    visited[ic] = true;

    let (op, value) = &instructions[ic];

    match &op[..] {
      "acc" => { acc += value }
      "jmp" => { ic = (ic as i32 + value - 1) as usize; }
      "nop" => { }
      _ => {}
    }

    ic += 1;
  }

  (acc, ic)
}

fn fix(instructions: &Vec<(String, i32)>) -> i32 {
  let filtered_iter = instructions
    .iter()
    .enumerate()
    .filter(|(_, (op, _))| &op[..] == "jmp" || &op[..] == "nop");

  for (idx, (op, _)) in filtered_iter {
    let mut instructions = instructions.clone();

    match &op[..] {
      "jmp" => { instructions[idx].0 = "nop".to_string() }
      "nop" => { instructions[idx].0 = "jmp".to_string() }
      _ => { }
    }

    let (acc, ic) = execute(&instructions);
    if ic == instructions.len() {
      return acc;
    }
  }

  0
}

fn load(filepath: &str) -> Vec<(String, i32)> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split("\n")
    .map(|entry| {
      (entry[..3].to_string(), entry[4..].parse::<i32>().unwrap())
    })
    .collect()
}

#[test]
fn test_load() {
  let loaded = load("inputs/day8/example.txt");
  let expected = vec![
    ("nop".to_string(), 0),
    ("acc".to_string(), 1),
    ("jmp".to_string(), 4),
    ("acc".to_string(), 3),
    ("jmp".to_string(), -3),
    ("acc".to_string(), -99),
    ("acc".to_string(), 1),
    ("jmp".to_string(), -4),
    ("acc".to_string(), 6),
  ];

  assert_eq!(loaded, expected);
}

#[test]
fn test_example() {
  let instructions = load("inputs/day8/example.txt");

  assert_eq!(execute(&instructions).0, 5);
}