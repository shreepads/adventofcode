// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn median_completion_score(file_path: &String) -> u64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect(&format!("Something went wrong reading the file {}", file_path));

    let mut completion_scores: Vec<u64> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        match completion_score(i, line) {
            Some(score) => completion_scores.push(score),
            None        => {},  
        }
    }

    calculate_median(&mut completion_scores)
}

fn completion_score(i: usize, line: &str) -> Option<u64> {

    let mut stack: Vec<char> = Vec::new();

    for chr in line.chars() {
        if i == 28 {
            //println!("Stack: {:?}", stack);
        }

        match chr {
            '(' | '[' | '{' | '<' => stack.push(chr),
            ')' | ']' | '}' | '>' => {
                let pop = match stack.pop() {
                    Some(x) => x,
                    None => '$', // incomplete line ignore
                };

                if pop == '$' {
                    println!("Incomplete line (missing open): {}", line);
                    return None;
                }

                if !valid_close(pop, chr) { // corrupt line ignore
                    return None;
                }
            }
            _ => {
                println!("Invalid chr: {}", chr);
                return None;
            }
        }
    }

    if stack.len() == 0 {
        println!("COMPLETE line: {}", line);
        return None;
    }

    stack.reverse();
    
    Some(stack.iter().fold(0u64, |acc, x| acc*5 + completion_points(*x)))

}

fn completion_points(open: char) -> u64 {
    // Completion points for closing open chars popped from the stack
    match open {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => {
            println!("Invalid open: {}", open);
            u64::MAX
        }
    }
}

fn valid_close(open: char, close: char) -> bool {
    if open == '(' && close == ')' {
        return true;
    }

    if open == '[' && close == ']' {
        return true;
    }

    if open == '{' && close == '}' {
        return true;
    }

    if open == '<' && close == '>' {
        return true;
    }

    false
}


// FIXED - from Day 7 #1
fn calculate_median(numbers: &mut Vec<u64>) -> u64 {
    numbers.sort_unstable();

    if numbers.len() % 2 == 1 {
        return numbers[((numbers.len() + 1) / 2) - 1];
    }

    return (numbers[(numbers.len() / 2) - 1] + numbers[((numbers.len() + 1) / 2) - 1]) / 2;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day10_2_works() {
        let result = median_completion_score(&String::from("../resources/tests/day10-2-testdata.txt"));
        assert_eq!(result, 288957);
    }
}
