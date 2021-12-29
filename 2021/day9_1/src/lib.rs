// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_risk_lowpoints(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut grid: Vec<Vec<i8>> = Vec::new();

    for (line_no, line_str) in contents.lines().enumerate() {
        if line_no == 0 {
            grid.push(vec![i8::MAX; line_str.len() + 2]);
        }

        let mut line: Vec<i8> = Vec::new();

        line.push(i8::MAX);

        for digit in line_str.chars() {
            line.push(digit.to_digit(10).unwrap().try_into().unwrap());
        }

        line.push(i8::MAX);

        grid.push(line);
    }

    grid.push(vec![i8::MAX; grid[0].len()]);

    //println!("Grid: \n {:?}", grid);

    let mut lowpoints_risk = 0u32;

    for (row_no, row) in grid.iter().enumerate() {
        if row_no == 0 || row_no == grid.len() - 1 {
            continue;
        }

        for (col_no, height) in row.iter().enumerate() {
            if col_no == 0 || col_no == row.len() - 1 {
                continue;
            }

            if local_min(&grid, row_no, col_no) {
                lowpoints_risk += (height + 1) as u32;
            }
        }
    }

    lowpoints_risk
}

fn local_min(grid: &Vec<Vec<i8>>, row_no: usize, col_no: usize) -> bool {
    let digit = grid[row_no][col_no];

    if digit >= grid[row_no - 1][col_no] {
        return false;
    }

    if digit >= grid[row_no + 1][col_no] {
        return false;
    }

    if digit >= grid[row_no][col_no - 1] {
        return false;
    }

    if digit >= grid[row_no][col_no + 1] {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
