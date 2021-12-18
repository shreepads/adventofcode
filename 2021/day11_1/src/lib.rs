// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_total_flashes(file_path: &String, rounds: u32) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect(&format!("Something went wrong reading the file {}", file_path));

    let mut total_flashes = 0u32;

    let mut grid : [ [u32; 12 ] ; 12];

    total_flashes
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
