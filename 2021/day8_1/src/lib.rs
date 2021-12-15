// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn count_output_digits(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut output_digits = 0u32;

    for line in contents.lines() {
        for (i, line_part) in line.split(" | ").enumerate() {
            if i == 0 {
                continue;
            }

            for output in line_part.split(" ") {
                match output.len() {
                    2 | 4 | 3 | 7 => output_digits += 1,
                    _             => {},
                }
            }
        }
    }
    
    output_digits
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
