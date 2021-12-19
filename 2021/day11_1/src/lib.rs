// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_total_flashes(file_path: &String, rounds: u32) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut total_flashes = 0u32;

    let mut grid: [[i32; 12]; 12] = [[i32::MIN; 12]; 12];

    load_data(&mut grid, contents);

    //println!("Grid: {:?}", grid);

    for _ in 1..=rounds {
        let flashes = boost(&mut grid);
        total_flashes += flashes;
    }

    total_flashes
}

fn boost(grid: &mut [[i32; 12]; 12]) -> u32 {
    let mut flashes = 0u32;

    let mut boosted_octos: Vec<(usize, usize)> = Vec::new();

    for (row_no, row) in grid.iter_mut().enumerate() {
        for (col_no, energy) in row.iter_mut().enumerate() {
            *energy += 1;
            if *energy == 10 {
                boosted_octos.push((row_no, col_no));
            }
        }
    }

    if boosted_octos.len() == 0 {
        return 0;
    }

    while boosted_octos.len() > 0 {
        let (row_no, col_no) = boosted_octos.pop().unwrap();
        flashes += 1;

        // boost adjacent octos
        for adj_row_no in row_no - 1..=row_no + 1 {
            for adj_col_no in col_no - 1..=col_no + 1 {
                if adj_row_no == row_no && adj_col_no == col_no {
                    continue; // not adjacent octo
                }

                grid[adj_row_no][adj_col_no] += 1;
                if grid[adj_row_no][adj_col_no] == 10 {
                    // don't boost if already boosted before
                    boosted_octos.push((adj_row_no, adj_col_no));
                }
            }
        }
    }

    // Set boosted back to 0 energy
    for row in grid.iter_mut() {
        for energy in row.iter_mut() {
            if *energy > 9 {
                *energy = 0;
            }
        }
    }

    flashes
}

fn load_data(grid: &mut [[i32; 12]; 12], contents: String) {
    for (row_no, row) in contents.lines().enumerate() {
        for (col_no, digit) in row.chars().enumerate() {
            grid[row_no + 1][col_no + 1] = digit.to_digit(10).unwrap() as i32;
        }
    }
}

fn _print_grid(grid: [[i32; 12]; 12]) {
    for row_no in 1..10 {
        for col_no in 1..10 {
            print!("{} ", grid[row_no][col_no]);
        }
        println!("");
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
