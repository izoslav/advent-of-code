use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn calculate_steps(
    src: &str,
    dst: &str,
    instructions: &[char],
    nodes: &HashMap<String, Node>,
) -> usize {
    let mut current = src.to_string();
    let mut step = 0;

    loop {
        if current.ends_with(dst) {
            break;
        }

        let idx = step % instructions.len();
        let instruction = instructions[idx];

        current = if instruction == 'L' {
            nodes[&current].left.clone()
        } else {
            nodes[&current].right.clone()
        };

        step += 1;
    }

    step
}

fn main() {
    let inputs = fs::read_to_string("inputs/day08.txt").unwrap();

    let instructions = inputs
        .lines()
        .take(1)
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    let nodes = inputs
        .lines()
        .skip(2)
        .map(|line| {
            let name = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();

            let node = Node { left, right };

            (name, node)
        })
        .collect::<HashMap<String, Node>>();

    let result = calculate_steps("AAA", "ZZZ", &instructions, &nodes);
    println!("Result part 1: {result}");

    let result = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|src| calculate_steps(&src, "Z", &instructions, &nodes))
        .fold(1, |acc, next| num::integer::lcm(acc, next));

    println!("Result part 2: {result}");
}
