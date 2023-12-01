use regex::Regex;

fn replace_number(input: &str) -> usize {
    match input {
        "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();

    let result = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|numbers| {
            let first = numbers.chars().nth(0).unwrap();
            let last = numbers.chars().nth_back(0).unwrap();
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum::<usize>();

    println!("Result part 1: {result}");

    let two_results = Regex::new(
        r"^.*?([0-9]|one|two|three|four|five|six|seven|eight|nine).*([0-9]|one|two|three|four|five|six|seven|eight|nine).*$",
    ).unwrap();

    let one_result = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let result = input
        .lines()
        .map(|line| {
            if let Some(caps) = two_results.captures(line) {
                let first = replace_number(&caps[1]);
                let last = replace_number(&caps[caps.len() - 1]);

                first * 10 + last
            } else {
                let caps = one_result.captures(line).unwrap();
                let number = replace_number(&caps[1]);

                number * 10 + number
            }
        })
        .sum::<usize>();

    println!("Result part 2: {result}");
}
