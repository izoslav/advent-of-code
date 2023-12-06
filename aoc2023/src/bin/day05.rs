use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Range {
    dst: i64,
    src: i64,
    len: i64,
}

impl Range {
    fn new(input: &str) -> Self {
        let input = input
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Self {
            dst: input[0],
            src: input[1],
            len: input[2],
        }
    }

    fn contains(&self, n: i64) -> bool {
        n >= self.src && n < self.src + self.len
    }
}

#[derive(Debug)]
struct RangeMapper {
    ranges: Vec<Range>,
}

impl RangeMapper {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn map(&self, n: i64) -> i64 {
        for range in &self.ranges {
            if range.contains(n) {
                return range.dst + n - range.src;
            }
        }

        n
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").unwrap();

    let seeds = input
        .lines()
        .take(1)
        .flat_map(|line| {
            let colon = line.find(':').unwrap();

            line[colon + 2..line.len()]
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<i64>>();

    let range_mappers = input
        .lines()
        .skip(2)
        .map(|line| format!("{line}\n"))
        .collect::<String>()
        .split("\n\n")
        .map(|block| {
            let ranges = block
                .lines()
                .skip(1)
                .map(|line| Range::new(line))
                .collect::<Vec<Range>>();

            RangeMapper::new(ranges)
        })
        .collect::<Vec<RangeMapper>>();

    println!("{seeds:?}");

    let result = &seeds
        .iter()
        .map(|seed| {
            let mut seed = *seed;
            range_mappers.iter().for_each(|rm| seed = rm.map(seed));

            seed
        })
        .min()
        .unwrap();

    println!("Result part 1: {result}");

    let seeds = seeds
        .into_iter()
        .tuples()
        .flat_map(|(start, end)| start..(start + end))
        .collect::<HashSet<i64>>();

    println!("{seeds:?}");

    let mapped_seeds = &seeds
        .par_iter()
        .map(|seed| {
            let mut seed = *seed;
            range_mappers.iter().for_each(|rm| seed = rm.map(seed));

            seed
        })
        .collect::<HashSet<i64>>();

    let result = mapped_seeds.intersection(&seeds).min().unwrap();

    println!("{result:?}");
}
