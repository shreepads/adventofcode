// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn three_largest_circuits_mul(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_largest_circuits_mul() {
        let result =
            three_largest_circuits_mul(&String::from("../resources/test-input/day08-test.txt"));
        assert_eq!(result, 40);
    }
}
