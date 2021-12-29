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

    let mut sfish_no_strs: Vec<String> = Vec::new();

    for line in contents.lines() {
        sfish_no_strs.push(line.to_string());
    }

    let mut max_magnitude = 0u32;

    for (no1, sno1_str) in sfish_no_strs.iter().enumerate() {
        for (no2, sno2_str) in sfish_no_strs.iter().enumerate() {
            if no1 == no2 {
                continue;
            }

            let mut sno1 = Number::new(sno1_str.to_string());
            let sno2 = Number::new(sno2_str.to_string());

            sno1.add(&sno2);
            sno1.reduce();

            if sno1.magnitude() > max_magnitude {
                max_magnitude = sno1.magnitude();
            }
        }
    }

    max_magnitude
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
