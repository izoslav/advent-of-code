use std::fs;

use array2d::Array2D;

struct Dumbo {
    energy_level: usize,
    flashed: bool,
}

impl Dumbo {
    fn new() -> Self {
        Dumbo {
            energy_level: 0,
            flashed: false,
        }
    }

    fn power_up(&mut self) {
        self.energy_level += 1;
    }

    fn reset(&mut self) {
        if self.energy_level > 9 {
            self.energy_level = 0;
        }

        self.flashed = false;
    }
}

impl Clone for Dumbo {
    fn clone(&self) -> Self {
        Dumbo {
            energy_level: self.energy_level,
            flashed: self.flashed,
        }
    }
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const STEPS: usize = 100;

fn main() {
    let mut octopi = Array2D::filled_with(Dumbo::new(), HEIGHT, WIDTH);

    fs::read_to_string("res/day11.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, n)| {
                octopi[(y, x)].energy_level = n.to_string().parse().unwrap();
            })
        });

    {
        // part 1
        let mut octopi = octopi.clone();
        let mut total_flashes = 0;

        for _ in 0..STEPS {
            total_flashes += step(&mut octopi);
        }

        println!("Part 1 answer: {}", total_flashes);
    }

    {
        // part 2
        for s in 0.. {
            step(&mut &mut octopi);

            if count_zeroes(&octopi) == 100 {
                println!("Part 2 answer: {}", s + 1);
                break;
            }
        }
    }
}

fn step(octopi: &mut Array2D<Dumbo>) -> usize {
    // power up
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            octopi[(y, x)].power_up();
        }
    }

    // flash
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            flash(octopi, x, y, false);
        }
    }

    let flashes = count_flashes(octopi);

    // zero
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            octopi[(y, x)].reset();
        }
    }

    flashes
}

fn flash(octopi: &mut Array2D<Dumbo>, x: usize, y: usize, inc: bool) {
    let mut octopus = octopi.get_mut(y, x).unwrap();

    if octopus.flashed {
        return;
    }

    if inc {
        octopus.power_up();
    }

    if octopus.energy_level > 9 {
        octopus.flashed = true;

        let left = x.checked_sub(1).is_some();
        let right = x + 1 < WIDTH;

        let top = y.checked_sub(1).is_some();
        let bottom = y + 1 < HEIGHT;

        if left && top {
            flash(octopi, x - 1, y - 1, true)
        };
        if right && top {
            flash(octopi, x + 1, y - 1, true)
        };
        if left && bottom {
            flash(octopi, x - 1, y + 1, true)
        };
        if right && bottom {
            flash(octopi, x + 1, y + 1, true)
        };

        if left {
            flash(octopi, x - 1, y, true)
        };
        if right {
            flash(octopi, x + 1, y, true)
        };
        if top {
            flash(octopi, x, y - 1, true)
        };
        if bottom {
            flash(octopi, x, y + 1, true)
        };
    }
}

fn count_flashes(octopi: &Array2D<Dumbo>) -> usize {
    octopi
        .elements_row_major_iter()
        .filter(|&octopus| octopus.flashed)
        .count()
}

fn count_zeroes(octopi: &Array2D<Dumbo>) -> usize {
    octopi
        .elements_row_major_iter()
        .filter(|&octopus| octopus.energy_level == 0)
        .count()
}
