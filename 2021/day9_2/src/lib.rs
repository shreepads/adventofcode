// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod graph;

use std::fs;
use graph::Graph;

pub fn top3_basins_product(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut grid : Vec<Vec<i8>> = Vec::new();

    for (line_no, line_str) in contents.lines().enumerate() {

        if line_no == 0 {
            grid.push(vec![9; line_str.len() + 2]);
        }

        let mut line : Vec<i8> = Vec::new();

        line.push(9);

        for digit in line_str.chars() {
            line.push(digit.to_digit(10).unwrap().try_into().unwrap());
        }

        line.push(9);

        grid.push(line);

    }

    grid.push(vec![9; grid[0].len()]);

    //println!("Grid: \n {:?}", grid);

    let mut graph = Graph::new();

    0u32

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
