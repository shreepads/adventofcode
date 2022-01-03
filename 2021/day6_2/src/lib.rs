// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_fish_population(file_path: &String, number_days: usize) -> u64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // Setup zero counts for all states
    let mut fishy_state_counts: [u64; 9] = [0; 9];
    let mut new_fishy_state_counts: [u64; 9] = [0; 9];

    for fishy_state_str in contents.trim_end().split(",") {
        let fishy_state = fishy_state_str.parse::<usize>().unwrap();

        fishy_state_counts[fishy_state] += 1;
    }

    for _ in 1..=number_days {
        for (fishy_state, fishy_statecount) in fishy_state_counts.iter().enumerate() {
            if fishy_state == 0 {
                new_fishy_state_counts[6] = *fishy_statecount;
                new_fishy_state_counts[8] = *fishy_statecount;
            } else if fishy_state == 7 {
                new_fishy_state_counts[6] += *fishy_statecount;
            } else {
                new_fishy_state_counts[fishy_state - 1] = *fishy_statecount;
            }
        }

        for (new_fishy_state, new_fishy_statecount) in new_fishy_state_counts.iter().enumerate() {
            fishy_state_counts[new_fishy_state] = *new_fishy_statecount;
        }
    }

    fishy_state_counts.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small_fishes() {
        let result =
            calculate_fish_population(&String::from("../resources/tests/day6-2-testdata.txt"), 256);

        assert_eq!(result, 26984457539);
    }
}
