// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub enum Location {
    Empty,
    East,
    South,
}

pub fn calculate_stop_steps(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut map: Vec<Vec<Location>> = Vec::new();

    load_map(&mut map, contents);

    print_map(&map);

    0
}

fn load_map(map: &mut Vec<Vec<Location>>, contents: String) {

    use crate::Location::*;

    for line in contents.lines() {
        
        let mut row : Vec<Location> = Vec::new();

        for loc in line.chars() {
            match loc {
                '>' => row.push(East),
                'v' => row.push(South),
                '.' => row.push(Empty),
                _   => println!("Invalid location {} on line {}", loc, line),
            }
        }

        map.push(row);
    }
}

fn print_map(map: &Vec<Vec<Location>>) {

    use crate::Location::*;

    for row in map.iter() {
        for loc in row.iter() {
            match loc {
                Empty => print!("."),
                South => print!("v"),
                East  => print!(">"),
                _     => print!("X"),
            }
        }
        println!("");
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn small() {
        let result = calculate_stop_steps(&String::from("../resources/tests/day25-1-testdata.txt"));
        assert_eq!(result, 58);
    }
}
