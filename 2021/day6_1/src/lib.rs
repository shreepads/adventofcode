// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_fish_population(file_path: &String, number_days: u32) -> usize {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut fishy_states: Vec<u8> = Vec::new();

    for fishy_state in contents.trim_end().split(",") {
        fishy_states.push(fishy_state.parse::<u8>().unwrap());
    }

    for _ in 1..=number_days {
        update_fishy_states(&mut fishy_states);
    }

    fishy_states.len()
}

fn update_fishy_states(fishy_states: &mut Vec<u8>) {
    let mut new_fishes = 0;

    for fishy_state in fishy_states.iter_mut() {
        if *fishy_state == 0 {
            *fishy_state = 6;
            new_fishes += 1;
            continue;
        }

        *fishy_state -= 1;
    }

    fishy_states.append(&mut vec![8; new_fishes]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
