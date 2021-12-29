// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_min_energy(file_path: &String) -> u32 {
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
    fn burrow_test() {
        let result = calculate_min_energy(
            &String::from("../resources/tests/day23-2-testdata.txt")
        );
        assert_eq!(result, 44169);
    }

}
