use std::fs;

use array2d::Array2D;

fn main() {
    let mut input = fs::read_to_string("res/day04.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>();

    let numbers = input[0]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();

    input.remove(0);

    let boards = input
        .chunks(5)
        .map(|lines| {
            let numbers = lines
                .iter()
                .map(|line| {
                    line.split(' ')
                        .filter(|&n| !n.is_empty())
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            Array2D::from_rows(&numbers)
        })
        .collect::<Vec<Array2D<usize>>>();

    {
        // part 1
        let mut boards = boards.clone();

        for n in &numbers {
            boards.iter_mut().for_each(|board| mark(board, *n));

            if let Some(score) = check_winner(&boards) {
                println!("Part 1 answer: {} ({} * {})", score * n, score, n);
                break;
            }
        }
    }

    {
        // part 2
        let mut boards = boards;

        for n in &numbers {
            boards.iter_mut().for_each(|board| mark(board, *n));

            if boards.len() != 1 {
                boards = boards
                    .into_iter()
                    .filter(|board| !check_board(board))
                    .collect();
            } else if let Some(score) = check_winner(&boards) {
                println!("Part 2 answer: {} ({} * {})", score * n, score, n);
                break;
            }
        }
    }
}

fn mark(board: &mut Array2D<usize>, number: usize) {
    for x in 0..5 {
        for y in 0..5 {
            if board[(x, y)] == number {
                board[(x, y)] = 100;
            }
        }
    }
}

fn check_board(board: &Array2D<usize>) -> bool {
    let number = 100;

    if board.rows_iter().any(|mut row| row.all(|n| *n == number)) {
        return true;
    }

    if board
        .columns_iter()
        .any(|mut column| column.all(|n| *n == number))
    {
        return true;
    }

    false
}

fn check_winner(boards: &[Array2D<usize>]) -> Option<usize> {
    for board in boards {
        if check_board(board) {
            let score = board
                .elements_row_major_iter()
                .filter(|&n| *n != 100)
                .sum::<usize>();

            return Some(score);
        }
    }

    None
}
