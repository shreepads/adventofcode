// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod space;

use std::fs;
use space::Cuboid;

pub fn calculate_cubes_on(file_path: &String, max_limit: i32) -> u32 {
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
    fn tiny_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-1-testdata1.txt"), 50);
        assert_eq!(result, 39);
    }


    #[test]
    fn small_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-1-testdata2.txt"), 50);
        assert_eq!(result, 590784);
    }
}
