// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn median_completion_score(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut completion_scores: Vec<u32> = Vec::new();

    for line in contents.lines() {
        match completion_score(line) {
            Some(score) => completion_scores.push(score),
            None        => {}  // ignore complete or corrupt lines,
        }
    }

    calculate_median(&mut completion_scores)
}

fn completion_score(line: &str) -> Option<u32> {
    Some(3)
}

// From Day 7 #1
fn calculate_median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort_unstable();

    if numbers.len() % 2 == 1 {
        return numbers[numbers.len() / 2 + 1];
    }

    return (numbers[numbers.len() / 2] + numbers[numbers.len() / 2 + 1]) / 2;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
