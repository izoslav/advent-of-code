use std::collections::BTreeSet;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

fn main() {
    let input = fs::read_to_string("res/day13.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut dots = input[0]
        .lines()
        .map(|line| {
            let coords = line.split(',').collect::<Vec<&str>>();
            let x = coords[0].parse::<usize>().unwrap();
            let y = coords[1].parse::<usize>().unwrap();

            (x, y)
        })
        .collect::<BTreeSet<(usize, usize)>>();

    let folds = input[1]
        .lines()
        .map(|line| line.split(' ').nth(2).unwrap())
        .map(|fold| {
            let fold = fold.split('=').collect::<Vec<&str>>();
            let direction = fold[0];
            let coord = fold[1].parse::<usize>().unwrap();

            match direction {
                "y" => Fold::Up(coord),
                "x" => Fold::Left(coord),
                _ => panic!(),
            }
        })
        .collect::<Vec<Fold>>();

    let (max_x, max_y) = max_coords(&dots);
    let width = max_x + 1;
    let height = max_y + 1;

    println!("Size: {}x{}", width, height);
    println!("Dots: {:?}", dots);
    println!("Folds: {:?}", folds);

    fold(&mut dots, folds[0]);

    println!("Part 1 answer: {}", dots.len());

    folds.iter().skip(1).for_each(|&f| fold(&mut dots, f));

    println!("Part 2 answer:");
    print_dots(&dots);
}

fn max_coords(dots: &BTreeSet<(usize, usize)>) -> (usize, usize) {
    let max_x = dots
        .iter()
        .max_by(|&(x1, _), &(x2, _)| x1.cmp(x2))
        .unwrap()
        .0;

    let max_y = dots
        .iter()
        .max_by(|&(_, y1), &(_, y2)| y1.cmp(y2))
        .unwrap()
        .1;

    (max_x, max_y)
}

fn fold(dots: &mut BTreeSet<(usize, usize)>, fold: Fold) {
    match fold {
        Fold::Up(y) => fold_up(dots, y),
        Fold::Left(x) => fold_left(dots, x),
    }
}

fn fold_up(dots: &mut BTreeSet<(usize, usize)>, fold_y: usize) {
    let mut updated = dots
        .iter()
        .filter(|&&(_, y)| y < fold_y)
        .map(|dot| dot.to_owned())
        .collect::<BTreeSet<(usize, usize)>>();

    let mirrored = dots
        .iter()
        .filter(|&&(_, y)| y > fold_y)
        .map(|dot| dot.to_owned())
        .collect::<BTreeSet<(usize, usize)>>();

    for (x, y) in mirrored {
        updated.insert((x, 2 * fold_y - y));
    }

    *dots = updated;
}

fn fold_left(dots: &mut BTreeSet<(usize, usize)>, fold_x: usize) {
    let mut updated = dots
        .iter()
        .filter(|&&(x, _)| x < fold_x)
        .map(|dot| dot.to_owned())
        .collect::<BTreeSet<(usize, usize)>>();

    let mirrored = dots
        .iter()
        .filter(|&&(x, _)| x > fold_x)
        .map(|dot| dot.to_owned())
        .collect::<BTreeSet<(usize, usize)>>();

    for (x, y) in mirrored {
        updated.insert((2 * fold_x - x, y));
    }

    *dots = updated;
}

fn print_dots(dots: &BTreeSet<(usize, usize)>) {
    let (max_x, max_y) = max_coords(dots);

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            print!("{}", if dots.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}
