use std::fs;

fn main() {
    let input = fs::read_to_string("res/day17.txt").unwrap();

    let min_x = input[15..18].parse::<i32>().unwrap();
    let max_x = input[20..23].parse::<i32>().unwrap();
    let min_y = input[27..30].parse::<i32>().unwrap();
    let max_y = input[32..35].parse::<i32>().unwrap();

    let max_altitude = (-min_y - 1) * -min_y / 2;
    println!("Part 1 answer: {}", max_altitude);

    let mut viable_starts = 0;

    for vx in 0..max_x + 1 {
        for vy in min_y - 1..100 {
            let mut vx = vx;
            let mut vy = vy;
            let mut x = vx;
            let mut y = vy;

            loop {
                if (min_x..max_x + 1).contains(&x) && (min_y..max_y + 1).contains(&y) {
                    viable_starts += 1;
                    break;
                }

                if (vx == 0 && !(min_x..max_x + 1).contains(&x)) || x > max_x || y < min_y {
                    break;
                }

                vy -= 1;
                vx -= if vx > 0 { 1 } else { 0 };

                x += vx;
                y += vy;
            }
        }
    }

    println!("Part 2 answer: {}", viable_starts);
}
