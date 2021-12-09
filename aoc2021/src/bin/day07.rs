use std::fs;

fn main() {
    let positions = fs::read_to_string("res/day07.txt")
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!(
        "Part 1 answer: {}",
        find_optimal_fuel_usage(&positions, &normal_cost)
    );
    println!(
        "Part 2 answer: {}",
        find_optimal_fuel_usage(&positions, &increasing_cost)
    );
}

fn find_optimal_fuel_usage(positions: &[i32], func: &'static dyn Fn(i32, i32) -> i32) -> i32 {
    let (min, max) = find_minmax(positions);

    let mut min_fuel = i32::MAX;
    for i in min..=max {
        let fuel = positions.iter().map(|&p| func(p, i)).sum();

        min_fuel = min_fuel.min(fuel);
    }

    min_fuel
}

fn find_minmax(input: &[i32]) -> (i32, i32) {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min, max)
}

fn normal_cost(src: i32, dst: i32) -> i32 {
    (dst - src).abs()
}

fn increasing_cost(src: i32, dst: i32) -> i32 {
    let max_cost = normal_cost(src, dst) as f32;

    ((1f32 + max_cost) / 2f32 * max_cost) as i32
}
