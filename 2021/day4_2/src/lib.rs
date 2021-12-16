// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use day4_1::board::Board;
use std::fs;

pub fn calculate_last_winning_board_score(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut numbers_called: Vec<u32> = Vec::new();

    let mut boards: Vec<Board> = Vec::new();

    load_data(&mut numbers_called, &mut boards, contents);

    let mut last_winning_board = Board::new_empty();

    let mut last_winning_number = 0u32;

    for number_called in numbers_called {
        for board in boards.iter_mut() {
            if board.mark_called_number(number_called) {
                last_winning_board = *board;
                last_winning_number = number_called;
            }
        }
    }

    last_winning_number * last_winning_board.score()
}

fn load_data(numbers_called: &mut Vec<u32>, boards: &mut Vec<Board>, contents: String) {
    let mut board_lines: [&str; 5] = [""; 5];

    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            // load called numbers
            for called_no in line.split(",") {
                numbers_called.push(called_no.parse::<u32>().unwrap());
            }
            continue;
        }

        if i % 6 == 1 {
            // skip blank line
            continue;
        }

        if (2..6).contains(&(i % 6)) {
            // load one of the first 4 lines
            board_lines[i % 6 - 2] = line;
            continue;
        }

        if i % 6 == 0 {
            // load last line and construct board
            board_lines[4] = line;
            let board = Board::new(board_lines);
            //println!("Loaded board: {:?}", board);
            boards.push(board);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
