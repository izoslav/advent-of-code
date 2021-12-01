use std::fs;

use ndarray::Array2;

use regex::Regex;

pub fn run() {
  let (colors, matrix) = load("inputs/day7/input.txt");
  let shiny_gold = colors.iter().position(|c| c == "shiny gold").unwrap();

  println!("=== Day  7 ===");
  println!("Colors that may contain shiny gold: {}", count_containing_colors(shiny_gold, &matrix));
  println!("Shiny gold bag contains : {} bags", count_bags_inside(shiny_gold, &matrix));
}

fn count_bags_inside(color_idx: usize, matrix: &Array2<usize>) -> usize {
  matrix
    .row(color_idx)
    .indexed_iter()
    .map(|(idx, value)| {
      if *value != 0 {
        value * (1 + count_bags_inside(idx, &matrix))
      }
      else { 0 }
    })
    .sum()
}

fn count_containing_colors(color_idx: usize, matrix: &Array2<usize>) -> usize {
  let colors_count = matrix.dim().0;
  let mut output = vec![false; colors_count];

  for i in 0..colors_count {
    count_containing_colors_step(i, i, color_idx, &matrix, &mut output);
  }

  output.iter().filter(|x| **x).count()
}

fn count_containing_colors_step(
  origin: usize,
  current: usize,
  color_idx: usize,
  matrix: &Array2<usize>,
  mut output: &mut Vec<bool>) -> () {
    if output[origin] == true || matrix[[current, color_idx]] > 0 {
      output[origin] = true;
      return;
    }

    matrix
      .row(current)
      .indexed_iter()
      .for_each(|(idx, value)| {
          if *value != 0 {
            count_containing_colors_step(
              origin,
              idx,
              color_idx,
              &matrix,
              &mut output
            )
          }
      })
}

// NOPE!
#[allow(unused)]
fn count_containing_colors_routing_table(color_idx: usize, matrix: Array2<usize>) -> usize {
  let size = matrix.dim().0;
  let mut matrix = matrix;
  let mut containing = vec![false; size];

  for i in 0..size {
    if matrix[[i, color_idx]] > 0 {
      containing[i] = true;
    }
  }

  let iterations = (size as f64).sqrt().floor() as usize;

  for iteration in 0..iterations {
    println!("Iteration {} / {}", iteration, iterations);

    let temp = matrix.clone();
    matrix = matrix.dot(&temp);

    for i in 0..size {
      if matrix[[i, color_idx]] > 0 {
        containing[i] = true;
      }
    }
  }

  containing.iter().filter(|x| **x).count()
}

fn load(filepath: &str) -> (Vec<String>, Array2<usize>) {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  let rules: Vec<String> = contents
    .trim()
    .split("\n")
    .map(|line| line.to_string())
    .collect();

  let re = Regex::new(r"(.+?) bags contain (.+)\.").unwrap();

  let colors: Vec<String> = rules
    .iter()
    .map(|rule| {
      let cap = re.captures(&rule).unwrap();
      cap[1].to_string()
    })
    .collect();

  let matrix: Vec<Vec<usize>> = rules
    .iter()
    .map(|rule| {
      let cap = re.captures(&rule).unwrap();
      let mut v = Vec::<usize>::new();
      v.resize(colors.len(), 0);

      if &cap[2] != "no other bags" {
        &cap[2]
          .split(',')
          .for_each(|bag| {
            let bag = bag.trim();

            let n = bag[..1].parse::<usize>().unwrap();
            let c = bag[1..bag.len() - 4].trim();
            let i = colors.iter().position(|color| color == c).unwrap();

            v[i] = n;
          });
      }

      v
    })
    .collect();

  let matrix = matrix.iter().flatten().cloned().collect();
  let size = colors.len();

  (colors, Array2::from_shape_vec((size, size), matrix).unwrap())
}

#[test]
fn test_contains() {
  let (colors, matrix) = load("inputs/day7/example_contains.txt");
  let shiny_gold = colors.iter().position(|c| c == "shiny gold").unwrap();

  assert_eq!(count_containing_colors(shiny_gold, &matrix), 4);
}

#[test]
fn test_count() {
  let (colors, matrix) = load("inputs/day7/example_count.txt");
  let shiny_gold = colors.iter().position(|c| c == "shiny gold").unwrap();

  assert_eq!(count_bags_inside(shiny_gold, &matrix), 126);
}