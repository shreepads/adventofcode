// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_fish_population(file_path: &String, _number_days: u32) -> u64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut fishy_states: Vec<u8> = Vec::new();

    for fishy_state in contents.trim_end().split(",") {
        fishy_states.push(fishy_state.parse::<u8>().unwrap());
    }

    // How many fishies are in the sea?
    // u64::MAX == 18,446,744,073,709,551,615

    let fish_count: u64 = fishy_states.len().try_into().unwrap();

    fish_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
