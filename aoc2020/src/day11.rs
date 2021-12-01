use std::fs;
use std::cmp;

pub fn run() {
  let input = load("inputs/day11/input.txt");

  println!("=== Day 11 ===");
  println!("Adjacent: {}", simulate(&input, 4, true));
  println!("Visible: {}", simulate(&input, 5, false));
}

fn simulate(seats: &Vec<Vec<char>>, num: usize, adjacent: bool) -> usize {
  let mut seats = seats.clone();
  let mut change = true;

  while change {
    let prev = seats.clone();
    change = false;

    for x in 0..seats.len() {
      for y in 0..seats[x].len() {
        if simulate_seat(&mut seats, &prev, x, y, num, adjacent) {
          change = true;
        }
      }
    }
  }

  seats
    .iter()
    .flat_map(|x| x.into_iter())
    .filter(|&x| *x == '#')
    .count()
}

fn simulate_seat(seats: &mut Vec<Vec<char>>, prev: &Vec<Vec<char>>, xsrc: usize, ysrc: usize, num: usize, adjacent: bool) -> bool {
  if seats[xsrc][ysrc] == '.' { return false; }

  let occupied = if adjacent {
    count_adjacent(&prev, xsrc, ysrc)
  } else {
    count_visible(&prev, xsrc, ysrc)
  };

  match seats[xsrc][ysrc] {
    'L' => {
      if occupied == 0 {
        seats[xsrc][ysrc] = '#';
        true
      }
      else {
        false
      }
    }
    '#' => {
      if occupied >= num {
        seats[xsrc][ysrc] = 'L';
        true
      }
      else {
        false
      }
    }
    _ => { false }
  }
}

fn count_adjacent(seats: &Vec<Vec<char>>, xsrc: usize, ysrc: usize) -> usize {
  let xsize = seats.len() as i32;
  let ysize = seats[0].len() as i32;
  let mut occupied = 0;

  for x in cmp::max(0, xsrc as i32 - 1)..cmp::min(xsize, xsrc as i32 + 2) {
    for y in cmp::max(0, ysrc as i32 - 1)..cmp::min(ysize, ysrc as i32 + 2) {
      if seats[x as usize][y as usize] == '#' {
        let xsrc = xsrc as i32;
        let ysrc = ysrc as i32;

        if x != xsrc || y != ysrc {
          occupied += 1;
        }
      }
    }
  }

  occupied
}

fn count_visible(seats: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
  if seats[x][y] == '.' { return 0; }

  let xsize = seats.len() as i32;
  let ysize = seats[0].len() as i32;
  let mut occupied = 0;

  for step in vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
    let mut x = x as i32 + step.0;
    let mut y = y as i32 + step.1;

    while x >= 0 && x < xsize && y >= 0 && y < ysize {
      let seat = seats[x as usize][y as usize];

      if seat != '.' {
        if seat == '#' {
          occupied += 1;
        }
        break;
      }

      x += step.0;
      y += step.1;
    }
  }

  occupied
}

fn load(filepath: &str) -> Vec<Vec<char>> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|line| {
      line
        .chars()
        .collect()
    })
    .collect()
}

#[test]
fn test_example() {
  let input = load("inputs/day11/example.txt");
  assert_eq!(simulate(&input, 4, true), 37);
  assert_eq!(simulate(&input, 5, false), 26);
}
