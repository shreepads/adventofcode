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

pub fn correct_answer_total(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let (numbers, ops) = correctly_load_problems(file_contents);

    assert_eq!(numbers.len(), ops.len());

    let mut ans_total = 0;

    for i in 0..numbers.len() {
        let ans = if ops[i] == '*' {
            numbers[i].iter().fold(1, |acc, i| acc * i)
        } else {
            numbers[i].iter().fold(0, |acc, i| acc + i)
        };

        ans_total += ans;
    }

    ans_total
}

fn correctly_load_problems(file_contents: String) -> (Vec<Vec<u64>>, Vec<char>) {
    //let mut problems = vec![];

    let mut numbers = vec![];
    let mut ops = vec![];

    let mut raw_data: Vec<Vec<char>> = vec![];

    for line in file_contents.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        raw_data.push(line_chars);
    }

    // Check all same length
    assert_eq!(raw_data[0].len(), raw_data[raw_data.len() - 1].len());

    let mut nums = vec![];

    // Read from the end by column, insert trimmed lines
    for i in (0..raw_data[0].len()).rev() {
        let mut line = String::from("");

        for j in 0..raw_data.len() {
            if !raw_data[j][i].is_whitespace() {
                line.push(raw_data[j][i]);
            }
        }

        if line.is_empty() {
            continue;
        }

        // Last number of problem ends with op code
        if line.ends_with(&['*', '+']) {
            let num = line.split_at(line.len() - 1).0.parse::<u64>().unwrap();
            nums.push(num);
            numbers.push(nums);

            ops.push(line.chars().nth(line.len() - 1).unwrap());
            nums = vec![];
            continue;
        }

        // Just a number
        nums.push(line.parse::<u64>().unwrap());

        //problems.push(line);
    }

    (numbers, ops)
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
    fn test_correctly_load_problems() {
        let result = correctly_load_problems(String::from(
            r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#,
        ));
        assert_eq!(
            result,
            (
                vec!(
                    vec!(4, 431, 623),
                    vec!(175, 581, 32),
                    vec!(8, 248, 369),
                    vec!(356, 24, 1),
                ),
                vec!('+', '*', '+', '*')
            )
        );
    }

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
