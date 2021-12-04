use std::fs;

use itertools::Itertools;

fn main() {
  let input = fs::read_to_string("res/day03.txt")
    .unwrap()
    .lines()
    .map(|l| l.to_string())
    .collect::<Vec<String>>();
  
  let mut bits = vec![(0, 0); input[0].len()];

  input
    .iter()
    .for_each(|l| {
      l.char_indices().for_each(|(i, c)| match c {
        '0' => bits[i].0 += 1,
        '1' => bits[i].1 += 1,
        _ => panic!()
      })
    });
  
  let gamma_binary = bits
    .iter()
    .fold("".to_string(), |acc, &(zeroes, ones)| acc + if ones > zeroes { "1" } else { "0" });

  let gamma = i32::from_str_radix(&gamma_binary, 2).unwrap();
  
  let epsilon_binary = bits
    .iter()
    .fold("".to_string(), |acc, &(zeroes, ones)| acc + if ones < zeroes { "1" } else { "0" });
  
  let epsilon = i32::from_str_radix(&epsilon_binary, 2).unwrap();

  println!("Part 1 answer: {}", gamma * epsilon);

  let mut o2_candidates = input.clone();
  let mut co2_candidates = input.clone();

  for i in 0..input[0].len() {
    let bits: Vec<char> = o2_candidates.iter().map(|n| n.chars().nth(i).unwrap()).collect();
    let zeroes = bits.iter().filter(|b| **b == '0').count();
    let ones = bits.iter().filter(|b| **b == '1').count();
    let most_common = if ones >= zeroes { '1' } else { '0' };

    if o2_candidates.len() != 1 {
      o2_candidates = o2_candidates.into_iter().filter(|n| n.chars().nth(i).unwrap() == most_common).collect::<Vec<String>>();
    }

    let bits: Vec<char> = co2_candidates.iter().map(|n| n.chars().nth(i).unwrap()).collect();
    let zeroes = bits.iter().filter(|b| **b == '0').count();
    let ones = bits.iter().filter(|b| **b == '1').count();
    let least_common = if ones < zeroes { '1' } else { '0' };
    
    if co2_candidates.len() != 1 {
      co2_candidates = co2_candidates.into_iter().filter(|n| n.chars().nth(i).unwrap() == least_common).collect::<Vec<String>>();
    }
  }

  let o2_generator_rating = i32::from_str_radix(&o2_candidates[0], 2).unwrap();
  let co2_scrubber_rating = i32::from_str_radix(&co2_candidates[0], 2).unwrap();

  println!("Part 2 answers: {}", o2_generator_rating * co2_scrubber_rating);
}
