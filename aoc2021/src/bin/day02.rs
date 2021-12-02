use std::fs;

fn main() {
    let commands = fs::read_to_string("res/day02.txt").unwrap();

    let (h, d) = dive(&commands, (0, 0), |(h, d), (dir, n)| match dir {
        "forward" => (h + n, d),
        "down" => (h, d + n),
        "up" => (h, d - n),
        _ => panic!(),
    });

    println!("Part 1 answer: {}", h * d);

    let (h, d, _) = dive(&commands, (0, 0, 0), |(h, d, a), (dir, n)| match dir {
        "forward" => (h + n, d + n * a, a),
        "down" => (h, d, a + n),
        "up" => (h, d, a - n),
        _ => panic!(),
    });

    println!("Part 2 answer: {}", h * d);
}

fn dive<F, Args>(commands: &str, init: Args, f: F) -> Args
where
    F: Fn(Args, (&str, i32)) -> Args,
{
    commands
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();

            (l.next().unwrap(), l.next().unwrap().parse::<i32>().unwrap())
        })
        .into_iter()
        .fold(init, f)
}
