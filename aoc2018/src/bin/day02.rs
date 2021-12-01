use std::fs;

struct BoxID {
    pub id: String,
    pub has_twos: bool,
    pub has_threes: bool,
}

impl BoxID {
    fn new(id: &str) -> BoxID {
        let (has_twos, has_threes) = check_multiples(id);

        BoxID {
            id: id.to_string(),
            has_twos,
            has_threes,
        }
    }
}

pub fn main() {
    let box_ids: Vec<BoxID> = fs::read_to_string("res/day02_1.txt")
        .unwrap()
        .lines()
        .map(|l| BoxID::new(l))
        .collect();

    let (twos, threes) = box_ids.iter().fold((0, 0), |(twos, threes), box_id| {
        (
            if box_id.has_twos { twos + 1 } else { twos },
            if box_id.has_threes {
                threes + 1
            } else {
                threes
            },
        )
    });

    println!("Part 1 checksum: {} * {} = {}", twos, threes, twos * threes);

    if let Some((id1, id2)) = find_matching_boxes(&box_ids) {
        println!("{}", id1);
        println!("{}", id2);
        println!("{}", common_letters(&id1, &id2));
    }
}

fn check_multiples(input: &str) -> (bool, bool) {
    let mut unique: Vec<char> = input.chars().collect::<Vec<char>>();
    unique.sort();
    unique.dedup();

    (
        unique.iter().any(|c| input.matches(*c).count() == 2),
        unique.iter().any(|c| input.matches(*c).count() == 3),
    )
}

fn find_matching_boxes(boxes: &Vec<BoxID>) -> Option<(String, String)> {
    let box_count = boxes.len();
    for i in 0..box_count - 1 {
        for j in i + 1..box_count {
            if do_boxes_match(&boxes[i], &boxes[j]) {
                return Some((boxes[i].id.clone(), boxes[j].id.clone()));
            }
        }
    }

    None
}

fn do_boxes_match(b1: &BoxID, b2: &BoxID) -> bool {
    let mut mismatches = 0;
    let length = b1.id.len();

    for i in 0..length {
        if b1.id.chars().nth(i) == b2.id.chars().nth(i) {
            continue;
        } else {
            mismatches += 1;

            if mismatches > 1 {
                return false;
            }
        }
    }

    true
}

fn common_letters(id1: &str, id2: &str) -> String {
    let mut output = String::new();
    let length = id1.len();

    for i in 0..length {
        if id1[i..i + 1] == id2[i..i + 1] {
            output += &id1[i..i + 1];
        }
    }

    output
}
