// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

#[derive(Debug, Clone)]
pub struct Position {
    pub horizontal: i64,
    pub depth: i64,
}

pub fn calculate_position(start_posn: Position, file_path: &String) -> Position {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let commands = contents.lines();

    let mut final_posn = start_posn.clone();

    for command in commands {
        final_posn = translate_position(final_posn, command);
    }

    final_posn
}

fn translate_position(posn: Position, command: &str) -> Position {
    let mut new_posn = posn.clone();

    let mut iter = command.split_whitespace();

    let instruction = iter.next();

    let magnitude: i64 = match iter.next() {
        Some(x) => x.parse::<i64>().unwrap(),
        None => {
            println!("Missing magnitude in command: {}", command);
            0
        }
    };

    match instruction {
        Some("forward") => new_posn.horizontal += magnitude,
        Some("down") => new_posn.depth += magnitude,
        Some("up") => new_posn.depth -= magnitude,
        _ => println!("Invalid instruction: {:?}", instruction),
    };

    // Submarines can't fly
    if new_posn.depth < 0 {
        new_posn.depth = 0
    };

    new_posn
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
