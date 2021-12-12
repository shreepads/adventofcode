// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod board;

use std::fs;
use self::board::Board;

pub fn calculate_winning_board_score(file_path: &String) -> u32 {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut numbers_called: Vec<u32> = Vec::new();

    let mut boards: Vec<Board> = Vec::new();

    load_data(&mut numbers_called, &mut boards, contents);

    10345u32
}

fn load_data(numbers_called: &mut Vec<u32>, boards: &mut Vec<Board>, contents: String) {


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
