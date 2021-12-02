use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

fn main() {
    let coords = fs::read_to_string("res/day06.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let split = l.split(", ").collect::<Vec<&str>>();

            (
                split[0].parse::<usize>().unwrap(),
                split[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    // normalize the coords
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;

    for (x, y) in &coords {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }
    max_x -= min_x;
    max_y -= min_y;

    // normalize
    // id, x, y
    let points: Vec<(usize, usize, usize)> = coords
        .iter()
        .enumerate()
        .map(|(i, (x, y))| (i + 1, x - min_x, y - min_y))
        .collect();

    // map
    let mut areas = BTreeMap::<usize, usize>::new();
    let mut infinites = BTreeSet::<usize>::new();

    for x in 0..max_x + 1 {
        for y in 0..max_y + 1 {
            // (distance, id)
            let mut min_distance = usize::MAX;
            let mut distances = BTreeMap::<usize, Vec<usize>>::new();

            for &(id, px, py) in &points {
                let distance = distance(x, y, px, py);
                min_distance = min_distance.min(distance);

                distances.entry(distance).or_insert(vec![]).push(id);
            }

            if distances[&min_distance].len() == 1 {
                let min_distance_id = distances[&min_distance][0];
                *areas.entry(min_distance_id).or_default() += 1;

                if x == 0 || y == 0 || x == max_x || y == max_y {
                    infinites.insert(min_distance_id);
                }
            }
        }
    }

    for infinite in infinites {
        areas.remove(&infinite);
    }

    let (_, largest_area) = areas
        .into_iter()
        .max_by(|&(_, a), &(_, b)| a.cmp(&b))
        .unwrap();

    println!("Part 1 answer: {}", largest_area);

    let mut safe_area = 0;
    for x in 0..max_x + 1 {
        for y in 0..max_y + 1 {
            let distance_sum = points
                .iter()
                .map(|&(_, px, py)| distance(x, y, px, py))
                .sum::<usize>();

            if distance_sum < 10000 {
                safe_area += 1;
            }
        }
    }

    println!("Part 2 answer: {}", safe_area);
}

fn distance(sx: usize, sy: usize, dx: usize, dy: usize) -> usize {
    sx.max(dx) - sx.min(dx) + sy.max(dy) - sy.min(dy)
}
