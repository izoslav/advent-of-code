use std::collections::VecDeque;

fn main() {
    let players = 426;
    let max_marble = 72058;

    println!("Part 1 answer: {}", max_score(players, max_marble));
    println!("Part 1 answer: {}", max_score(players, max_marble * 100));
}

fn max_score(players: usize, max_marble: usize) -> usize {
    let mut player = 0;
    let mut marbles = VecDeque::with_capacity(max_marble);
    let mut scores = vec![0; players];

    marbles.push_back(0);
    marbles.push_back(1);

    for m in 2..max_marble {
        if m % 23 == 0 {
            marbles.rotate_right(7);
            scores[player] += m + marbles.pop_front().unwrap();
        } else {
            marbles.rotate_left(2);
            marbles.insert(0, m);
        }

        player = (player + 1) % players;
    }

    scores.iter().max().unwrap().to_owned()
}
