// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod grid;

use std::fs;
use grid::Grid;

pub fn calculate_overlap_points(file_path: &String) -> u32 {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut grid = Grid::new();

    load_lines(&mut grid, contents);

    0u32
}

fn load_lines(grid: &mut Grid, contents: String) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
