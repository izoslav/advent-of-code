use std::fs;

use array2d::Array2D;
use regex::Regex;

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Claim {
    fn new(entry: &str) -> Self {
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

        let captures = re.captures(entry).unwrap();

        if let &[id, x, y, w, h] = captures
            .iter()
            .skip(1)
            .take(5)
            .map(|option| option.unwrap())
            .map(|c| c.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .as_slice()
        {
            return Claim { id, x, y, w, h };
        } else {
            panic!()
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

fn main() {
    let entries: Vec<String> = fs::read_to_string("res/day03.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let claims: Vec<Claim> = entries.into_iter().map(|e| Claim::new(&e)).collect();

    let mut fabric = Array2D::filled_with(0i32, 1000, 1000);

    apply_claims(&claims, &mut fabric);

    let multiclaimed: usize = fabric
        .rows_iter()
        .map(|row| row.into_iter().filter(|e| **e == -1).count())
        .sum();

    println!(
        "Part 1 answer: {} square inches are claimed by multiple elves",
        multiclaimed
    );

    let intact_claim_id = find_intact_claim(&claims, &fabric);

    println!("Part 2 answer: intact claim has id #{}", intact_claim_id);
}

fn apply_claims(claims: &Vec<Claim>, fabric: &mut Array2D<i32>) {
    for claim in claims {
        let (x, y) = claim.coords();
        let (w, h) = claim.size();

        for i in x..x + w {
            for j in y..y + h {
                if fabric[(i, j)] == 0 {
                    fabric[(i, j)] = claim.id() as i32;
                } else {
                    fabric[(i, j)] = -1;
                }
            }
        }
    }
}

fn find_intact_claim(claims: &Vec<Claim>, fabric: &Array2D<i32>) -> i32 {
    for claim in claims {
        let (x, y) = claim.coords();
        let (w, h) = claim.size();
        let mut intact = true;

        for i in x..x + w {
            for j in y..y + h {
                if fabric[(i, j)] != claim.id() as i32 {
                    intact = false;
                    break;
                }
            }

            if !intact {
                break;
            }
        }

        if intact {
            return claim.id() as i32;
        }
    }

    -1
}
