use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let (algorithm, image) = fs::read_to_string("res/day20.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect_tuple::<(String, String)>()
        .unwrap();

    let algorithm = algorithm
        .chars()
        .map(|c| if c == '#' { '1' } else { '0' })
        .collect_vec();

    let mut image = image
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(x, _)| (x as i32, y as i32))
                .collect_vec()
        })
        .flatten()
        .collect::<HashSet<(i32, i32)>>();

    image = enhance(2, &algorithm, image);
    println!("Part 1 answer: {}", image.len());
    image = enhance(48, &algorithm, image);
    println!("Part 2 answer: {}", image.len());
}

fn enhance(steps: usize, algorithm: &[char], image: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut image = image;

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for &(x, y) in &image {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    for step in 0..steps {
        let (exists_val, not_exists_val) = if step % 2 == 0 {
            ('1', '0')
        } else {
            ('0', '1')
        };

        let mut next_image = HashSet::<(i32, i32)>::new();
        let mut next_min_x = i32::MAX;
        let mut next_min_y = i32::MAX;
        let mut next_max_x = i32::MIN;
        let mut next_max_y = i32::MIN;

        for y in min_y - 1..=max_y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let mut buf = String::new();
                for j in y - 1..=y + 1 {
                    for i in x - 1..=x + 1 {
                        let c = if image.contains(&(i, j)) {
                            exists_val
                        } else {
                            not_exists_val
                        };
                        buf.push(c);
                    }
                }
                let index = usize::from_str_radix(&buf, 2).unwrap();
                let c = if step % 2 == 0 { '0' } else { '1' };

                if algorithm[index] == c {
                    next_image.insert((x, y));

                    next_min_x = next_min_x.min(x);
                    next_max_x = next_max_x.max(x);
                    next_min_y = next_min_y.min(y);
                    next_max_y = next_max_y.max(y);
                }
            }
        }

        image = next_image;
        min_x = next_min_x;
        max_x = next_max_x;
        min_y = next_min_y;
        max_y = next_max_y;
    }

    image
}
