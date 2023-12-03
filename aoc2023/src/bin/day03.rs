use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Number {
    x: u32,
    y: u32,
    value: u32,
    length: u32,
}

impl Number {
    fn new(x: u32, y: u32, value: u32) -> Self {
        Self {
            x,
            y,
            value,
            length: value.checked_ilog10().unwrap_or(0) + 1,
        }
    }

    fn is_adjacent(&self, x: u32, y: u32) -> bool {
        let min_x = self.x.checked_sub(1).unwrap_or(0);
        let max_x = self.x + self.length;
        let min_y = self.y.checked_sub(1).unwrap_or(0);
        let max_y = self.y + 1;

        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }
}

fn main() {
    let mut numbers = vec![];
    let mut symbols = HashMap::<(u32, u32), char>::new();

    let input = fs::read_to_string("inputs/day03.txt").unwrap();

    input.lines().enumerate().for_each(|(y, line)| {
        let mut current = 0;
        loop {
            if current >= line.len() {
                break;
            }

            let start = line[current..line.len()]
                .find(|c: char| c.is_digit(10))
                .unwrap_or(line.len());

            if start == line.len() {
                break;
            }

            let end = line[current + start..line.len()]
                .find(|c: char| !c.is_digit(10))
                .unwrap_or(line.len());

            let e = usize::min(line.len(), current + start + end);
            let value = line[current + start..e].parse::<u32>().unwrap();

            numbers.push(Number::new((current + start) as u32, y as u32, value));

            current = current + start + end;
        }

        line.chars()
            .enumerate()
            .filter(|&(_, c)| !c.is_digit(10) && c != '.')
            .for_each(|(x, c)| {
                symbols.insert((x as u32, y as u32), c);
            });
    });

    let res = numbers
        .iter()
        .filter(|n| {
            symbols.iter().any(|s| {
                let &(x, y) = s.0;
                n.is_adjacent(x, y)
            })
        })
        .map(|n| n.value)
        .sum::<u32>();

    println!("Result part 1: {res}");

    let res = symbols
        .iter()
        .filter(|&(_, c)| *c == '*')
        .map(|s| {
            let &(x, y) = s.0;

            let adjacent = numbers
                .iter()
                .filter(|n| n.is_adjacent(x, y))
                .map(|n| n.value)
                .collect::<Vec<u32>>();

            if adjacent.len() == 2 {
                adjacent[0] * adjacent[1]
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("Result part 2: {res}");
}
