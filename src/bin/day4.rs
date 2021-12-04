use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day4.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let random_numbers = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = contents
        .split("\n\n")
        .skip(1)
        .map(|b| {
            b.lines()
                .map(|l| {
                    l.split(" ")
                        .filter(|n| *n != "")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut numbers = HashSet::new();

    for n in random_numbers.iter() {
        numbers.insert(*n);
        let bingo_board = boards.iter().find(|board| check_bingo(board, &numbers));

        if bingo_board.is_some() {
            let board = bingo_board.unwrap();
            let sum = board.iter().fold(0, |sum, row| {
                row.iter().fold(sum, |sum, col| {
                    if numbers.contains(col) {
                        sum
                    } else {
                        sum + col
                    }
                })
            });
            println!("answer1 {}", sum * n);
            break;
        }
    }

    let mut numbers = HashSet::new();
    let mut last_board = None;
    let mut last_n = None;
    let mut last_numbers = None;

    for n in random_numbers.iter() {
        numbers.insert(*n);
        let (winning_boards, rest): (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<i32>>>) = boards
            .into_iter()
            .partition(|board| check_bingo(board, &numbers));

        winning_boards.iter().for_each(|bingo_board| {
            let board = bingo_board.clone();
            last_board = Some(board.clone());
            last_n = Some(n);
            last_numbers = Some(numbers.clone());
        });

        boards = rest;
    }

    let board = last_board.unwrap();
    let last_numbers = last_numbers.unwrap();
    let sum = board.iter().fold(0, |sum, row| {
        row.iter().fold(sum, |sum, col| {
            if last_numbers.contains(col) {
                sum
            } else {
                sum + col
            }
        })
    });
    println!("answer2 {}", sum * last_n.unwrap());

    Ok(())
}

fn check_bingo(board: &Vec<Vec<i32>>, numbers: &HashSet<i32>) -> bool {
    let row_bingo = board
        .iter()
        .any(|row| row.iter().all(|n| numbers.contains(n)));

    let col_bingo = (0..board[0].len())
        .any(|col_idx| (0..board.len()).all(|row_idx| numbers.contains(&board[row_idx][col_idx])));

    row_bingo || col_bingo
}
