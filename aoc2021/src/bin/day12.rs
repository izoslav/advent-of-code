use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

type Caves = HashMap<String, BTreeSet<String>>;

fn main() {
    let mut caves = Caves::new();

    fs::read_to_string("res/day12.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let delim = line.find('-').unwrap();

            (&line[0..delim], &line[delim + 1..])
        })
        .for_each(|(start, end)| {
            if !caves.contains_key(start) {
                caves.insert(start.to_string(), BTreeSet::new());
            }

            if !caves.contains_key(end) {
                caves.insert(end.to_string(), BTreeSet::new());
            }

            caves.get_mut(start).unwrap().insert(end.to_string());
            caves.get_mut(end).unwrap().insert(start.to_string());
        });

    println!("Part 1 answer: {}", count_routes(&caves, false));
    println!("Part 2 answer: {}", count_routes(&caves, true));
}

fn is_small(cave: &str) -> bool {
    cave == cave.to_lowercase()
}

fn count_routes(caves: &Caves, can_visit_twice: bool) -> usize {
    let mut visited = Vec::<String>::new();
    visit(caves, &mut visited, "start", !can_visit_twice)
}

fn visit(
    caves: &Caves,
    visited: &mut Vec<String>,
    current: &str,
    small_cave_twice_visited: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    if current == "start" && !visited.is_empty() {
        return 0;
    }

    let mut small_cave_twice_visited = small_cave_twice_visited;

    if is_small(current) && visited.contains(&current.to_string()) {
        if small_cave_twice_visited {
            return 0;
        } else {
            small_cave_twice_visited = true;
        }
    }

    visited.push(current.to_string());

    let mut acc = 0;
    for cave in &caves[current] {
        acc += visit(caves, visited, cave, small_cave_twice_visited);
    }

    visited.pop();

    acc
}
