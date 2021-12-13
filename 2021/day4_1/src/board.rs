// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    number: u32,
    marked: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    board_data: [[Cell; 5]; 5],
    board_won: bool,
}

impl Board {

    pub fn new_empty() -> Board {
        Board {
            board_data: [[Cell {number:0,marked:false}; 5]; 5],
            board_won: false,
        }
    }

    pub fn new(board_lines: [&str; 5]) -> Board {

        //println!("Constructing board with: {:?}", board_lines);

        let mut data: [[Cell; 5]; 5] = [[Cell {number:0,marked:false}; 5]; 5];

        for (row_no, line) in board_lines.iter().enumerate() {
            for (column_no, num) in line.split_whitespace().enumerate() {
                data[row_no][column_no] = Cell {
                    number: num.parse::<u32>().unwrap(),
                    marked: false,
                }
            }
        }

        Board {
            board_data : data,
            board_won: false,
        }
    }

    pub fn mark_called_number(&mut self, called_number: u32) -> bool {

        if self.board_won {
            // If board won stop marking further cells and declaring board won
            return false;
        }

        let mut found = false;
        let mut found_row_no = 0usize;
        let mut found_column_no = 0usize;
        
        'outer: for (row_no, row) in self.board_data.iter_mut().enumerate() {
            for (column_no, cell) in row.iter_mut().enumerate() {
                if cell.number == called_number {
                    cell.marked = true;
                    found = true;
                    found_column_no = column_no;
                    found_row_no = row_no;
                    break 'outer;
                }
            }
        }

        if found {
            if self.check_win(found_row_no, found_column_no) {
                self.board_won = true;
                return true;
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }

    fn check_win(&self, row_no: usize, column_no: usize) -> bool {
        
        let mut won = true;

        for cell in self.board_data[row_no].iter() {
            if !cell.marked {
                won = false;
                break;
            }
        }
        
        if won {
            return true;
        }

        won = true;

        for row in self.board_data.iter() {
            if !row[column_no].marked {
                won = false;
                break;
            }
        }

        won

    }

    pub fn score(&self) -> u32 {
        let mut score = 0u32;

        for row in self.board_data.iter() {
            score += row.iter()
                .filter(|x| !x.marked)
                .fold(0, |acc, x| acc + x.number);
        }

        score
    }
}