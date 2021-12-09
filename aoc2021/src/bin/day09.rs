use std::fs;

use array2d::Array2D;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("res/day09.txt").unwrap();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut heightmap = Array2D::filled_with(9, height + 2, width + 2);

    for (y, line) in input.trim().lines().enumerate() {
        for (x, n) in line
            .trim()
            .chars()
            .map(|n| n.to_string().parse::<usize>().unwrap())
            .enumerate()
        {
            *heightmap.get_mut(y + 1, x + 1).unwrap() = n;
        }
    }

    let mut risk_level = 0;
    let mut basins = Vec::<usize>::new();

    for y in 1..height + 1 {
        for x in 1..width + 1 {
            let north = *heightmap.get(y - 1, x).unwrap();
            let south = *heightmap.get(y + 1, x).unwrap();
            let west = *heightmap.get(y, x - 1).unwrap();
            let east = *heightmap.get(y, x + 1).unwrap();
            let current = *heightmap.get(y, x).unwrap();

            if [north, south, west, east].iter().all(|&p| current < p) {
                risk_level += current + 1;
                basins.push(check_basin(&mut heightmap, x, y));
            }
        }
    }

    let mul_area = basins
        .iter()
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();

    println!("Part 1 answer: {}", risk_level);
    println!("Part 2 answer: {}", mul_area);
}

fn check_basin(heightmap: &mut Array2D<usize>, x: usize, y: usize) -> usize {
    if *heightmap.get(y, x).unwrap() == 9 {
        return 0;
    }

    *heightmap.get_mut(y, x).unwrap() = 9usize;

    let north = check_basin(heightmap, x, y + 1);
    let south = check_basin(heightmap, x, y - 1);
    let east = check_basin(heightmap, x + 1, y);
    let west = check_basin(heightmap, x - 1, y);

    1 + north + south + east + west
}
