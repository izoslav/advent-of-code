use std::fs;

#[derive(PartialEq, Debug)]
struct Action {
  instruction: String,
  value: i32
}

#[derive(Debug)]
struct State {
  x: i32,
  y: i32,
  direction: i32
}

pub fn run() {
  let input = load("inputs/day12/input.txt");

  println!("=== Day 12 ===");
  println!("Manhattan: {}", manhattan_distance(input));

  let input = load("inputs/day12/input.txt");
  println!("With waypoint: {}", manhattan_with_waypoint(input));
}

fn manhattan_distance(actions: Vec<Action>) -> i32 {
  let mut state = State {x: 0, y:0, direction: 90};

  for action in actions {
    state = execute_action(action, state);
  }

  state.x.abs() + state.y.abs()
}

fn execute_action(action: Action, state: State) -> State {
  let Action {instruction, value} = action;
  let State {x, y, direction} = state;

  match &instruction[..] {
    "N" => { State { x, y: y + value, direction } }
    "S" => { State { x, y: y - value, direction } }
    "E" => { State { x: x + value, y, direction } }
    "W" => { State { x: x - value, y, direction } }
    "F" => { forward(Action { instruction, value }, state) }
    "L" => { turn(Action { instruction, value }, state) }
    "R" => { turn(Action { instruction, value }, state) }
    _ => State { x: 0, y: 0, direction: 0 }
  }
}

fn forward(action: Action, state: State) -> State {
  let Action {instruction: _, value} = action;
  let State {x, y, direction} = state;

  match direction {
    0 => { State { x, y: y + value, direction } }
    90 => { State { x: x + value, y, direction } }
    180 => { State { x, y: y - value, direction } }
    270 => { State { x: x - value, y, direction } }
    _ => panic!("Unknown direction")
  }
}

fn turn(action: Action, state: State) -> State {
  let Action {instruction, value} = action;
  let State {x, y, direction} = state;

  match &instruction[..] {
    "L" => { State{ x, y, direction: (360 + direction - value) % 360 }}
    "R" => { State{ x, y, direction: (direction + value) % 360 }}
    _ => panic!("Unknown turn")
  }
}

fn manhattan_with_waypoint(actions: Vec<Action>) -> i32 {
  let mut shipx = 0;
  let mut shipy = 0;
  let mut waypx = 10;
  let mut waypy = 1;

  for action in actions {
    let Action { instruction, value} = action;

    match &instruction[..] {
      "N" => { waypy += value }
      "S" => { waypy -= value }
      "E" => { waypx += value }
      "W" => { waypx -= value }
      "L" => {
        let (x, y) = rotate_waypoint(waypx, waypy, 360 - value);

        waypx = x;
        waypy = y;
      }
      "R" => {
        let (x, y) = rotate_waypoint(waypx, waypy, value);

        waypx = x;
        waypy = y;
      }
      "F" => {
        shipx += waypx * value;
        shipy += waypy * value;
      }
      _ => panic!("Unknown instruction")
    };
  }

  shipx.abs() + shipy.abs()
}

fn rotate_waypoint(x: i32, y: i32, degrees: i32) -> (i32, i32) {
  match degrees {
    0 => { (x, y) }
    90 => { (y, -x) }
    180 => { (-x, -y) }
    270 => { (-y, x) }
    _ => panic!("Unknown rotation")
  }
}

fn load(filepath: &str) -> Vec<Action> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|line| {
      Action {
        instruction: String::from(&line[0..1]),
        value: line[1..].parse::<i32>().unwrap()
      }
    })
    .collect()
}

#[test]
fn load_test() {
  let input = load("inputs/day12/example.txt");

  let expected = vec![
    Action {instruction: "F".to_string(), value: 10},
    Action {instruction: "N".to_string(), value: 3},
    Action {instruction: "F".to_string(), value: 7},
    Action {instruction: "R".to_string(), value: 90},
    Action {instruction: "F".to_string(), value: 11},
  ];

  assert_eq!(input, expected);
}

#[test]
fn example_first_test() {
  let input = load("inputs/day12/example.txt");

  assert_eq!(manhattan_distance(input), 25);
}

#[test]
fn example_second_test() {
  let input = load("inputs/day12/example.txt");

  assert_eq!(manhattan_with_waypoint(input), 286);
}