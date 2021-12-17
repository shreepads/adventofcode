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

    println!("Completion scores: {:?}", completion_scores);
    calculate_median(&mut completion_scores)
}

fn completion_score(line: &str) -> Option<u32> {

    let mut stack: Vec<char> = Vec::new();

    for chr in line.chars() {
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

                if !valid_close(pop, chr) {
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
    
    Some(stack.iter().fold(0u32, |acc, x| acc*5 + completion_points(*x)))

}

fn completion_points(open: char) -> u32 {
    // Completion points for closing open chars popped from the stack
    match open {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => {
            println!("Invalid open: {}", open);
            u32::MAX
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
