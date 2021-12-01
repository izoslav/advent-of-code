use std::fs;
use std::slice::Iter;

fn main() {
    let input: Vec<usize> = fs::read_to_string("res/day08_1.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|entry| entry.parse::<usize>().unwrap())
        .collect();

    let mut iter = input.iter();
    let sum = sum_metadata(&mut iter);
    println!("Part 1 answer: {:?}", sum);

    let mut iter = input.iter();
    let indexed_sum = sum_indexed_metadata(&mut iter);
    println!("Part 2 answer: {:?}", indexed_sum)
}

fn sum_metadata(iter: &mut Iter<usize>) -> usize {
    let mut sum = 0;

    let c = iter.next().unwrap();
    let m = iter.next().unwrap();

    for _ in 0..*c {
        sum += sum_metadata(iter);
    }

    for _ in 0..*m {
        sum += *iter.next().unwrap();
    }

    sum
}

fn sum_indexed_metadata(iter: &mut Iter<usize>) -> usize {
    let c = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut children_values = vec![];
    for _ in 0..*c {
        children_values.push(sum_indexed_metadata(iter));
    }

    let mut metadata = vec![];
    for _ in 0..*m {
        metadata.push(*iter.next().unwrap());
    }

    if *c == 0 {
        metadata.iter().sum()
    } else {
        let mut sum = 0;
        for i in 0..metadata.len() {
            let index = metadata[i] - 1;
            if index < children_values.len() {
                sum += children_values[index];
            }
        }

        sum
    }
}
