// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use day18_1::sfish_math::Number;
use std::fs;

pub fn calculate_maxmagnitude_sum(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
