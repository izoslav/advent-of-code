use std::fs;

fn main() {
    let polymer = fs::read_to_string("res/day05_1.txt").unwrap();

    let reduced_len = reduced_len(&polymer);
    println!("Part 1 answer: {}", reduced_len);

    remove_and_reduce(&polymer, 'k');

    let (_, length) = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| (c, remove_and_reduce(&polymer, c)))
        .min_by(|&a, &b| a.1.cmp(&b.1))
        .unwrap();

    println!("Part 2 answer: {}", length);
}

fn reduced_len(polymer: &str) -> usize {
    let mut polymer = polymer.clone().to_string();

    let mut i = 0;
    while i < polymer.len() - 1 {
        let lhs = &polymer[i..i + 1];
        let rhs = &polymer[i + 1..i + 2];

        if lhs != rhs && lhs.to_ascii_lowercase() == rhs.to_ascii_lowercase() {
            polymer.remove(i);
            polymer.remove(i);

            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    polymer.len()
}

fn remove_and_reduce(polymer: &str, unit_type: char) -> usize {
    let polymer = polymer
        .clone()
        .chars()
        .filter(|&c| c.to_ascii_lowercase() != unit_type)
        .collect::<String>();

    reduced_len(&polymer)
}
