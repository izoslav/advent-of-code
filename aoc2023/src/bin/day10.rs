use std::{collections::HashMap, fmt, fs};

#[derive(Debug, Clone, Copy)]
enum Pipe {
    Start,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            _ => panic!(),
        }
    }

    fn next(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        match self {
            Self::NS => vec![(x, y - 1), (x, y + 1)],
            Self::EW => vec![(x - 1, y), (x + 1, y)],
            Self::NE => vec![(x, y - 1), (x + 1, y)],
            Self::NW => vec![(x, y - 1), (x - 1, y)],
            Self::SW => vec![(x, y + 1), (x - 1, y)],
            Self::SE => vec![(x, y + 1), (x + 1, y)],
            _ => panic!(),
        }
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Pipe::Start => 'S',
            Pipe::NS => '|',
            Pipe::EW => '-',
            Pipe::NE => 'L',
            Pipe::NW => 'J',
            Pipe::SW => '7',
            Pipe::SE => 'F',
        };

        write!(f, "{symbol}")
    }
}

fn find_start(map: &HashMap<(i32, i32), Pipe>) -> (i32, i32) {
    *map.iter()
        .find(|&(_, value)| matches!(value, Pipe::Start))
        .unwrap()
        .0
}

fn replace_start(x: i32, y: i32, map: &mut HashMap<(i32, i32), Pipe>) {
    let n = if y > 0 {
        if let Some(north) = map.get(&(x, y - 1)) {
            matches!(north, Pipe::NS | Pipe::SW | Pipe::SE)
        } else {
            false
        }
    } else {
        false
    };

    let s = if let Some(south) = map.get(&(x, y + 1)) {
        matches!(south, Pipe::NS | Pipe::NW | Pipe::NE)
    } else {
        false
    };

    let e = if let Some(east) = map.get(&(x + 1, y)) {
        matches!(east, Pipe::EW | Pipe::NW | Pipe::SW)
    } else {
        false
    };

    let w = if x > 0 {
        if let Some(west) = map.get(&(x - 1, y)) {
            matches!(west, Pipe::EW | Pipe::NE | Pipe::SE)
        } else {
            false
        }
    } else {
        false
    };

    let replacement = match (n, s, e, w) {
        (true, true, false, false) => Pipe::NS,
        (true, false, true, false) => Pipe::NE,
        (true, false, false, true) => Pipe::NW,
        (false, false, true, true) => Pipe::EW,
        (false, true, true, false) => Pipe::SE,
        (false, true, false, true) => Pipe::SW,
        _ => panic!(),
    };

    map.insert((x, y), replacement);
}

fn calculate_distances(
    x: i32,
    y: i32,
    map: &HashMap<(i32, i32), Pipe>,
) -> HashMap<(i32, i32), i32> {
    let mut distances = HashMap::new();

    let mut step = 1;
    let current = (x, y);
    let mut frontier = map.get(&current).unwrap().next(x, y);

    while !frontier.is_empty() {
        let mut next_frontier = vec![];

        for (nx, ny) in frontier {
            if let Some(&distance) = distances.get(&(nx, ny)) {
                if distance <= step {
                    continue;
                }
            }

            distances.insert((nx, ny), step);
            next_frontier.append(&mut map.get(&(nx, ny)).unwrap().next(nx, ny));
        }

        step += 1;

        frontier = next_frontier;
    }

    distances
}

fn calculate_inner_area(map: &HashMap<(i32, i32), Pipe>) -> usize {
    let minx = map.keys().map(|&(x, _)| x).min().unwrap();
    let maxx = map.keys().map(|&(x, _)| x).max().unwrap();
    let miny = map.keys().map(|&(_, y)| y).min().unwrap();
    let maxy = map.keys().map(|&(_, y)| y).max().unwrap();

    let mut area = 0;

    for y in miny..=maxy {
        let mut hit_pipes = 0;
        let mut previous = Pipe::Start;

        for x in minx..=maxx {
            if let Some(pipe) = map.get(&(x, y)) {
                match pipe {
                    Pipe::NS => hit_pipes += 1,
                    Pipe::NE | Pipe::SE => previous = *pipe,
                    Pipe::SW if matches!(previous, Pipe::NE) => hit_pipes += 1,
                    Pipe::NW if matches!(previous, Pipe::SE) => hit_pipes += 1,
                    _ => {}
                }
            } else if hit_pipes > 0 && hit_pipes % 2 == 1 {
                area += 1;
            }
        }
    }

    area
}

fn main() {
    let mut map = fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .map(move |(x, c)| ((x as i32, y as i32), Pipe::from(c)))
        })
        .collect::<HashMap<(i32, i32), Pipe>>();

    let (x, y) = find_start(&map);
    replace_start(x, y, &mut map);

    let distances = calculate_distances(x, y, &map);
    let result = distances.values().max().unwrap();
    println!("Result part 1: {result}");

    let main_loop = map
        .into_iter()
        .filter(|(key, _)| distances.contains_key(key))
        .collect::<HashMap<(i32, i32), Pipe>>();

    let result = calculate_inner_area(&main_loop);
    println!("Result part 2: {result}");
}
