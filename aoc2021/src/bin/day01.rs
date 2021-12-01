use std::fs;

fn main() {
    let measurements = fs::read_to_string("res/day01.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let two_increses = measurements
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|&change| change > 0)
        .count();

    println!("Part 1 answer: {}", two_increses);

    let three_increases = measurements
        .windows(3)
        .map(|three| three.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .filter(|&change| change > 0)
        .count();

    println!("Part 2 answer: {}", three_increases);
}
