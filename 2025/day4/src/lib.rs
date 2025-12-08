// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn reachable_rolls(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let roll_grid = load_roll_grid(file_contents);

    count_reachable_rolls(roll_grid)
}

fn count_reachable_rolls(roll_grid: Vec<Vec<u8>>) -> u64 {
    let mut reachable_rolls = 0;

    for (row_id, roll_row) in roll_grid.iter().enumerate() {
        for (roll_id, roll) in roll_row.iter().enumerate() {
            if *roll == 1 {
                // Add up rolls around this one and check if < 4
                let mut count: u8 = 0;

                let left_id = if roll_id == 0 { 0 } else { roll_id - 1 };
                let right_id = if roll_id < roll_row.len() - 1 {
                    roll_id + 1
                } else {
                    roll_row.len() - 1
                };

                // Row above
                if row_id > 0 {
                    count += roll_grid[row_id - 1][left_id..right_id + 1]
                        .iter()
                        .sum::<u8>();
                }

                // Roll row
                count += roll_grid[row_id][left_id..right_id + 1].iter().sum::<u8>();

                // Row below
                if row_id < roll_grid.len() - 1 {
                    count += roll_grid[row_id + 1][left_id..right_id + 1]
                        .iter()
                        .sum::<u8>();
                }

                // Subtract the r0ll itself
                count -= 1;

                if count < 4 {
                    reachable_rolls += 1;
                }
            }
        }
    }

    reachable_rolls
}

fn load_roll_grid(file_contents: String) -> Vec<Vec<u8>> {
    let mut roll_grid = vec![];

    for line in file_contents.lines() {
        let roll_row = line.chars().map(|c| if c == '.' { 0 } else { 1 }).collect();

        roll_grid.push(roll_row);
    }

    roll_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reachable_rolls() {
        let result = reachable_rolls(&String::from("../resources/test-input/day04-test.txt"));
        assert_eq!(result, 13);
    }

    #[test]
    fn test_load_roll_grid() {
        let result = load_roll_grid(String::from(
            r#"..@.
@@.@
.@@.
...@
"#,
        ));
        assert_eq!(
            result,
            vec!(
                vec!(0, 0, 1, 0),
                vec!(1, 1, 0, 1),
                vec!(0, 1, 1, 0),
                vec!(0, 0, 0, 1)
            )
        );
    }
}
