// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_beacon_count(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    0
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day19_1() {
        let result = calculate_beacon_count(&String::from("../resources/tests/day19-1-testdata.txt"));
        assert_eq!(result, 79);
    }
}
