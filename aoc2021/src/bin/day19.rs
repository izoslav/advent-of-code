use std::{fs, collections::HashSet, hash::Hash};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

impl Point {
  fn new(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z, }
  }

  fn from(s: &str) -> Self {
    let coords = s
      .split(',')
      .map(|coord| coord.parse::<i32>().unwrap())
      .collect_vec();

    Self { x: coords[0], y: coords[1], z: coords[2] }
  }

  fn distance(&self, other: &Self) -> usize {
    let dx = other.x - self.x;
    let dy = other.y - self.y;
    let dz = other.z - self.z;

    ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f32).sqrt() as usize
  }
}

impl Ord for Point {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self.x == other.x {
      if self.y == other.y {
        self.z.cmp(&other.z)
      } else {
        self.y.cmp(&other.y)
      }
    } else {
      self.x.cmp(&other.x)
    }
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

fn main() {
  let beacons = fs::read_to_string("res/day19.txt")
    .unwrap()
    .split("\n\n")
    .map(|scanner| {
      scanner
        .lines()
        .skip(1)
        .map(|line| Point::from(line))
        .collect_vec()
    })
    .collect_vec();

  let beacon_distances = beacons
    .iter()
    .map(|beacons| {
      beacons
        .iter()
        .enumerate()
        .cartesian_product(beacons.iter().enumerate())
        .filter(|&((i1, _), (i2, _))| i1 < i2)
        .map(|((_, b1), (_, b2))| b1.distance(b2))
        .collect::<HashSet<usize>>()
    })
    .collect_vec();

  let overlapping = beacon_distances
    .iter()
    .enumerate()
    .cartesian_product(beacon_distances.iter().enumerate())
    .filter(|&((i1, _), (i2, _))| i1 < i2)
    .map(|((i1, s1), (i2, s2))| {
      (i1, i2, s1.intersection(s2).count())
    })
    .filter(|&(_, _, c)| c >= 66)
    .collect_vec();

  // println!("beacons: {:?}", beacons);
  // println!("beacon_distances: {:?}", beacon_distances);
  println!("overlapping: {:?}", overlapping);

  // println!("scanner 1: {:?}", beacon_distances[1]);
  // println!("scanner 4: {:?}", beacon_distances[4]);
}