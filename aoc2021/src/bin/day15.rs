use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    cost: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let map = fs::read_to_string("res/day15.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_string().parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    println!("Part 1 answer: {}", shortest_path(&map));

    let height = map.len();
    let width = map[0].len();

    let mut expanded_map = vec![vec![0; width * 5]; height * 5];

    for y in 0..height {
        for x in 0..width {
            for i in 0..5 {
                for j in 0..5 {
                    let v = map[y][x] + i + j;
                    let v = if v > 9 { v % 10 + 1 } else { v };
                    expanded_map[y + (height * i)][x + (width * j)] = v;
                }
            }
        }
    }

    println!("Part 2 answer: {}", shortest_path(&expanded_map));
}

fn shortest_path(map: &[Vec<usize>]) -> usize {
    let height = map.len();
    let width = map[0].len();
    let start = (0, 0);
    let goal = (width - 1, height - 1);

    let mut frontier = BinaryHeap::<Point>::new();
    let mut costs = HashMap::<(usize, usize), usize>::new();

    frontier.push(Point {
        x: start.0,
        y: start.1,
        cost: 0,
    });

    while let Some(Point { x, y, cost }) = frontier.pop() {
        if x == goal.0 && y == goal.1 {
            return cost;
        }

        if let Some(&target_cost) = costs.get(&(x, y)) {
            if cost > target_cost {
                continue;
            }
        }

        let mut neighbours = Vec::<Point>::new();

        if x > 0 {
            neighbours.push(Point {
                x: x - 1,
                y,
                cost: cost + map[y][x - 1],
            });
        }

        if x < width - 1 {
            neighbours.push(Point {
                x: x + 1,
                y,
                cost: cost + map[y][x + 1],
            });
        }

        if y > 0 {
            neighbours.push(Point {
                x,
                y: y - 1,
                cost: cost + map[y - 1][x],
            });
        }

        if y < height - 1 {
            neighbours.push(Point {
                x,
                y: y + 1,
                cost: cost + map[y + 1][x],
            });
        }

        for neighbour in neighbours {
            let next = Point {
                x: neighbour.x,
                y: neighbour.y,
                cost: map[neighbour.y][neighbour.x] + cost,
            };
            let prev_cost = if let Some(&v) = costs.get(&(next.x, next.y)) {
                v
            } else {
                usize::MAX
            };

            if next.cost < prev_cost {
                frontier.push(next);
                *costs.entry((next.x, next.y)).or_default() = next.cost;
            }
        }
    }

    panic!()
}
