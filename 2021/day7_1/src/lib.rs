// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_least_fuel(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut crab_posns: Vec<u32> = Vec::new();

    for crab_posn in contents.trim_end().split(",") {
        crab_posns.push(crab_posn.parse::<u32>().unwrap());
    }

    println!("Loaded crab posns: {:?}", crab_posns);

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
