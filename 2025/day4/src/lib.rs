// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn reachable_rolls(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let roll_grid = load_roll_grid(file_contents);

    count_reachable_rolls(roll_grid)
}

pub fn ultimately_reachable_rolls(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let roll_grid = load_roll_grid(file_contents);

    count_ultimately_reachable_rolls(roll_grid)
}

fn count_ultimately_reachable_rolls(roll_grid: Vec<Vec<u8>>) -> u64 {
    let mut ult_reachable_rolls = 0;

    let mut more_reachable_rolls = true;

    let mut curr_roll_grid = roll_grid.clone();
    let mut next_roll_grid = roll_grid.clone();

    while more_reachable_rolls {
        // Set more_reachable_rolls to false, change back to true if any removed
        more_reachable_rolls = false;

        // Work thru current roll grid and update next roll grid
        // Update more_reachable_rolls to true if any one updated
        for (row_id, roll_row) in curr_roll_grid.iter().enumerate() {
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
                        count += curr_roll_grid[row_id - 1][left_id..right_id + 1]
                            .iter()
                            .sum::<u8>();
                    }

                    // Roll row
                    count += curr_roll_grid[row_id][left_id..right_id + 1]
                        .iter()
                        .sum::<u8>();

                    // Row below
                    if row_id < roll_grid.len() - 1 {
                        count += curr_roll_grid[row_id + 1][left_id..right_id + 1]
                            .iter()
                            .sum::<u8>();
                    }

                    // Subtract the roll itself
                    count -= 1;

                    if count < 4 {
                        // Update next roll grid, ult_reachable_rolls and more_reachable_rolls
                        next_roll_grid[row_id][roll_id] = 0;
                        ult_reachable_rolls += 1;
                        more_reachable_rolls = true;
                    }
                }
            }
        }

        // Replace current roll grid with next roll grid
        // Performance impact of clone
        curr_roll_grid = next_roll_grid.clone();
    }

    ult_reachable_rolls
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

                // Subtract the roll itself
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
    fn test_ultimately_reachable_rolls() {
        let result =
            ultimately_reachable_rolls(&String::from("../resources/test-input/day04-test.txt"));
        assert_eq!(result, 43);
    }

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
