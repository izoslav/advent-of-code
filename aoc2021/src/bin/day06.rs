use std::fs;

fn main() {
    println!("Part 1 answer: {}", calculate_number_of_fish(80));
    println!("Part 2 answer: {}", calculate_number_of_fish(256));
}

fn calculate_number_of_fish(days: usize) -> usize {
    let reset_t = 6;
    let new_t = 8;

    let mut counts = vec![0; new_t + 1];

    fs::read_to_string("res/day06.txt")
        .unwrap()
        .split(',')
        .into_iter()
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| counts[n] += 1);

    for _ in 0..days {
        let children = counts[0];

        for t in 0..new_t {
            counts[t] = counts[t + 1];
        }

        counts[reset_t] += children;
        counts[new_t] = children;
    }

    counts.iter().sum()
}
