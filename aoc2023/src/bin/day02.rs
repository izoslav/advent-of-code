use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(input: &str) -> Self {
        let colon_pos = input.find(':').unwrap();
        let id = input[5..colon_pos].parse::<u32>().unwrap();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        input[colon_pos + 1..].split(';').for_each(|record| {
            record.split(',').for_each(|draw| {
                let draw = draw.trim();
                let space = draw.find(' ').unwrap();
                let value = draw[0..space].parse::<u32>().unwrap();
                match &draw[space + 1..space + 2] {
                    "r" => r = r.max(value),
                    "g" => g = g.max(value),
                    "b" => b = b.max(value),
                    _ => panic!(),
                }
            });
        });

        Game {
            id: id,
            red: r,
            green: g,
            blue: b,
        }
    }

    fn possible(&self, r: u32, g: u32, b: u32) -> bool {
        self.red <= r && self.green <= g && self.blue <= b
    }
}

fn main() {
    let games = fs::read_to_string("inputs/day02.txt")
        .unwrap()
        .lines()
        .map(|line| Game::new(line))
        .collect::<Vec<Game>>();

    let r1 = games
        .iter()
        .filter(|game| game.possible(12, 13, 14))
        .map(|game| game.id)
        .sum::<u32>();

    println!("Result part 1: {r1}");

    let r2 = games
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum::<u32>();

    println!("Result part 2: {r2}");
}
