// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn answer_total(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let (numbers, ops) = load_problems(file_contents);

    // Check lengths are the same
    assert_eq!(numbers[0].len(), ops.len());

    let mut sum = 0;

    for i in 0..ops.len() {
        if ops[i] == '+' {
            let mut answer = 0;
            for j in 0..numbers.len() {
                answer += numbers[j][i];
            }
            sum += answer;
        }

        if ops[i] == '*' {
            let mut answer = 1;
            for j in 0..numbers.len() {
                answer *= numbers[j][i];
            }
            sum += answer;
        }
    }

    sum
}

fn load_problems(file_contents: String) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut numbers = vec![];
    let mut ops = vec![];

    for line in file_contents.lines() {
        if line.starts_with(&['*', '+']) {
            let ops_iter = line
                .split_ascii_whitespace()
                .map(|x| x.chars().nth(0).unwrap());

            ops.extend(ops_iter);

            break;
        }

        // Number line
        let nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        numbers.push(nums);
    }

    (numbers, ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_problems() {
        let result = load_problems(String::from(
            r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#,
        ));
        assert_eq!(
            result,
            (
                vec!(
                    vec!(123, 328, 51, 64),
                    vec!(45, 64, 387, 23),
                    vec!(6, 98, 215, 314)
                ),
                vec!('*', '+', '*', '+')
            )
        );
    }
}
