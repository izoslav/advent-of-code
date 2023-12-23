use std::fs;

fn predict(history: &mut Vec<Vec<i32>>) {
    loop {
        let current = history.last().unwrap();

        if current.iter().all(|&e| e == 0) {
            break;
        }

        let next = current
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<i32>>();

        history.push(next);
    }
}

fn extrapolate_forward(history: &mut [Vec<i32>]) {
    history.last_mut().unwrap().push(0);
    let mut tail = 0;

    history.iter_mut().rev().skip(1).for_each(|prediction| {
        tail += prediction.last().unwrap();
        prediction.push(tail);
    });
}

fn extrapolate_backward(history: &mut [Vec<i32>]) {
    history.last_mut().unwrap().push(0);
    let mut head = 0;

    history.iter_mut().rev().skip(1).for_each(|prediction| {
        head = prediction.first().unwrap() - head;
        prediction.push(head);
        prediction.rotate_right(1);
    });
}

fn main() {
    let input = fs::read_to_string("inputs/day09.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let result = input
        .into_iter()
        .map(|history| {
            let mut history = vec![history];

            predict(&mut history);
            extrapolate_forward(&mut history);
            extrapolate_backward(&mut history);

            (*history[0].last().unwrap(), *history[0].first().unwrap())
        })
        .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2));

    println!("Result part 1: {}", result.0);
    println!("Result part 2: {}", result.1);
}
