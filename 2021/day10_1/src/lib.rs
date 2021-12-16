// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_total_syntaxerror_score(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    
    let mut total_syntaxerror_score = 0u32;

    for line in contents.lines() {
        total_syntaxerror_score += syntaxerror_score(line);
    }

    total_syntaxerror_score
}

fn syntaxerror_score(line: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
