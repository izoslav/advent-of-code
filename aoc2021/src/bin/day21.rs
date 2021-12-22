use std::collections::HashMap;

const DIRACDIEROLLS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn main() {
    let p1_start = 4;
    let p2_start = 9;

    println!("Part 1 answer: {}", normal_die(p1_start, p2_start));
    println!("Part 2 answer: {}", dirac_die(p1_start, p2_start));
}

fn normal_die(p1_start: usize, p2_start: usize) -> usize {
    let track_size = 10;

    let mut positions = vec![p1_start - 1, p2_start - 1];
    let mut scores = vec![0, 0];

    let mut roll = 1;

    loop {
        let active_player = (roll - 1) % 2;
        let moves = (roll * 3 + 3) % 100;

        roll += 3;

        positions[active_player] = (positions[active_player] + moves) % track_size;
        scores[active_player] += positions[active_player] + 1;

        if scores.iter().any(|&p| p >= 1000) {
            return scores.iter().min().unwrap() * (roll - 1);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    p1_pos: usize,
    p1_score: usize,
    p2_pos: usize,
    p2_score: usize,
    p1_turn: bool,
}

fn dirac_die(p1_start: usize, p2_start: usize) -> usize {
    let state = State {
        p1_pos: p1_start - 1,
        p1_score: 0,
        p2_pos: p2_start - 1,
        p2_score: 0,
        p1_turn: true,
    };
    let mut cache = HashMap::new();

    let (p1_wins, p2_wins) = dirac_die_step(state, &mut cache);

    p1_wins.max(p2_wins)
}

fn dirac_die_step(state: State, cache: &mut HashMap<State, (usize, usize)>) -> (usize, usize) {
    if state.p1_score >= 21 {
        return (1, 0);
    }

    if state.p2_score >= 21 {
        return (0, 1);
    }

    let mut total = (0, 0);

    for &(steps, frequency) in DIRACDIEROLLS.iter() {
        let mut next_state = state;

        if state.p1_turn {
            next_state.p1_pos = (state.p1_pos + steps) % 10;
            next_state.p1_score += next_state.p1_pos + 1;
            next_state.p1_turn = false;
        } else {
            next_state.p2_pos = (state.p2_pos + steps) % 10;
            next_state.p2_score += next_state.p2_pos + 1;
            next_state.p1_turn = true;
        }

        let (p1_wins, p2_wins) = match cache.get(&next_state) {
            Some(&res) => res,
            None => {
                let res = dirac_die_step(next_state, cache);
                cache.insert(next_state, res);
                res
            }
        };

        total.0 += p1_wins * frequency;
        total.1 += p2_wins * frequency;
    }

    total
}
