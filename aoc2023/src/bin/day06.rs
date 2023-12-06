use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn new(time: u64, record: u64) -> Self {
        Self { time, record }
    }

    fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .map(|t| (self.time - t) * t)
            .filter(|&d| d > self.record)
            .count() as u64
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day06.txt").unwrap();

    let re = Regex::new(r"  +").unwrap();
    let input = re.replace_all(&input, " ");

    let entries = input
        .split('\n')
        .map(|line| {
            let colon = line.find(':').unwrap();

            line[colon + 1..line.len()]
                .trim()
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let races = entries[0]
        .iter()
        .zip(entries[1].iter())
        .map(|(&t, &d)| Race::new(t, d))
        .collect::<Vec<Race>>();

    println!("Races: {races:?}");

    let result = races.iter().map(|r| r.ways_to_win()).product::<u64>();

    println!("Result part 1: {result}");

    let input = input.replace(" ", "");
    let entries = input.lines().collect::<Vec<&str>>();
    let time = entries[0][5..entries[0].len()].parse::<u64>().unwrap();
    let record = entries[1][9..entries[1].len()].parse::<u64>().unwrap();

    let result = Race::new(time, record).ways_to_win();

    println!("Result part 2: {result}");
}
