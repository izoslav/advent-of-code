use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("inputs/day11.txt").unwrap();
    let width = input.lines().next().unwrap().len();

    let galaxies = input
        .lines()
        .enumerate()
        .filter(|&(_, line)| line.contains('#'))
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(x, _)| (x, y))
                .collect::<HashSet<(usize, usize)>>()
        })
        .collect::<HashSet<(usize, usize)>>();

    let empty_rows = input
        .lines()
        .enumerate()
        .filter(|&(_, line)| !line.contains('#'))
        .map(|(y, _)| y)
        .collect::<HashSet<usize>>();

    let empty_columns = (0..width)
        .filter(|&x| !input.lines().any(|l| l.chars().nth(x).unwrap() == '#'))
        .collect::<HashSet<usize>>();

    let result = galaxies
        .iter()
        .map(|&(x1, y1)| {
            galaxies
                .iter()
                .filter(|&&(x2, y2)| !(x1 == x2 && y1 == y2))
                .map(|&(x2, y2)| {
                    let sx = x1.min(x2);
                    let sy = y1.min(y2);
                    let dx = x1.max(x2);
                    let dy = y1.max(y2);

                    let expanded_rows = empty_rows.iter().filter(|&&r| sy < r && r < dy).count();

                    let expanded_columns =
                        empty_columns.iter().filter(|&&c| sx < c && c < dx).count();

                    let d = (dx - sx) + (dy - sy);
                    let e = expanded_rows + expanded_columns;

                    (d + e, d + e * 999999)
                })
                .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2))
        })
        .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2));

    println!("Result part 1: {}", result.0 / 2);
    println!("Result part 2: {}", result.1 / 2)
}
