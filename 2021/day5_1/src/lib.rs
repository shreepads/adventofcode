// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod grid;

use grid::Grid;
use std::fs;

pub fn calculate_overlap_points(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut grid = Grid::new();

    load_lines(&mut grid, contents);

    grid.overlap_points()
}

fn load_lines(grid: &mut Grid, contents: String) {
    for line in contents.lines() {
        // println!("Adding line: {}", line);

        let mut x1 = 0usize;
        let mut y1 = 0usize;
        let mut x2 = 0usize;
        let mut y2 = 0usize;

        for (i, coords) in line.split(" -> ").enumerate() {
            if i == 0 {
                let mut coord = coords.split(",");
                x1 = coord.next().unwrap().parse::<usize>().unwrap();
                y1 = coord.next().unwrap().parse::<usize>().unwrap();
            } else {
                let mut coord = coords.split(",");
                x2 = coord.next().unwrap().parse::<usize>().unwrap();
                y2 = coord.next().unwrap().parse::<usize>().unwrap();
            }
        }

        grid.add_line(x1, y1, x2, y2, false);
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
