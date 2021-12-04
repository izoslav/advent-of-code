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

    let mut boards = input
        .chunks(5)
        .map(|lines| {
            let mut board: Array2D<usize> = Array2D::filled_with(0, 5, 5);

            for r in 0..5 {
                let numbers = lines[r]
                    .split(' ')
                    .filter(|&n| !n.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>();

                for c in 0..5 {
                    *board.get_mut(r, c).unwrap() = numbers[c];
                }
            }

            board
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
        let mut boards = boards.clone();

        for n in &numbers {
            boards.iter_mut().for_each(|board| mark(board, *n));

            if boards.len() != 1 {
                boards = boards
                    .into_iter()
                    .filter(|board| !check_board(board))
                    .collect();
            } else {
                if let Some(score) = check_winner(&boards) {
                    println!("Part 2 answer: {} ({} * {})", score * n, score, n);
                    break;
                }
            }
        }
    }
}

fn mark(board: &mut Array2D<usize>, number: usize) {
    for x in 0..5 {
        for y in 0..5 {
            let n = board.get_mut(x, y).unwrap();

            if *n == number {
                *n = 100;
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

    // let mut bingo = true;
    // for i in 0..5 {
    //   if *board.get(i, i).unwrap() != 100 {
    //     bingo = false;
    //     break;
    //   }
    // }
    // if bingo { return true; }

    // let mut bingo = true;
    // for i in 0..5 {
    //   if *board.get(4 - i, i).unwrap() != 100 {
    //     bingo = false;
    //     break;
    //   }
    // }
    // if bingo { return true; }

    false
}

fn check_winner(boards: &Vec<Array2D<usize>>) -> Option<usize> {
    for board in boards {
        if check_board(&board) {
            let score = board
                .elements_row_major_iter()
                .filter(|&n| *n != 100)
                .sum::<usize>();

            return Some(score);
        }
    }

    None
}
