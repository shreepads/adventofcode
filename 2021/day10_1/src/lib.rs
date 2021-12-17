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
    
    let mut stack: Vec<char> = Vec::new();

    for chr in line.chars() {
        match chr {
            '(' | '[' | '{' | '<' => stack.push(chr),
            ')' | ']' | '}' | '>' => {
                let pop = match stack.pop() {
                    Some(x) => x,
                    None    => '$',   // incomplete line ignore
                };

                if pop == '$' {
                    println!("Incomplete line (missing open): {}", line);
                    return 0;
                }

                if !valid_close(pop, chr) {
                    return invalid_close_score(chr);
                }
            },
            _ => println!("Invalid chr: {}", chr),
        }
    }

    if stack.len() == 0 {
        println!("COMPLETE line: {}", line);
    }

    0
}

fn valid_close(open: char, close: char) -> bool {
    if open == '(' &&  close == ')' {
        return true;
    }

    if open == '[' &&  close == ']' {
        return true;
    }

    if open == '{' &&  close == '}' {
        return true;
    }

    if open == '<' &&  close == '>' {
        return true;
    }

    false
}

fn invalid_close_score(close: char) -> u32 {
    match close {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _   => {
            println!("Invalid close: {}", close);
            u32::MAX
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
