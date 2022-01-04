// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;
use std::{thread, time};

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
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

    let mut done = false;
    let mut step = 1;

    while !done {
        done = seac_shuffle(&mut map);
        step += 1;
        //print_map(&map);
    }

    step - 1
}

fn seac_shuffle(map: &mut Vec<Vec<Location>>) -> bool {
    use crate::Location::*;

    let mut map_clone = map.clone();
    let mut shuffled = false;

    let row_count = map.len();
    let col_count = map[0].len();

    // Move East
    for (row_no, row) in map.iter().enumerate() {
        for (col_no, loc) in row.iter().enumerate() {
            let next_col_no = if col_no == (col_count - 1) {
                0
            } else {
                col_no + 1
            };

            let next_loc = map[row_no][next_col_no];

            if *loc == East && next_loc == Empty {
                map_clone[row_no][col_no] = Empty;
                map_clone[row_no][next_col_no] = East;
                shuffled = true;
            }
        }
    }

    let mut map_clone2 = map_clone.clone();

    // Move South
    for (row_no, row) in map_clone.iter().enumerate() {
        for (col_no, loc) in row.iter().enumerate() {
            let next_row_no = if row_no == (row_count - 1) {
                0
            } else {
                row_no + 1
            };

            let next_loc = map_clone[next_row_no][col_no];

            if *loc == South && next_loc == Empty {
                map_clone2[row_no][col_no] = Empty;
                map_clone2[next_row_no][col_no] = South;
                shuffled = true;
            }
        }
    }

    // Reassign clone to main
    *map = map_clone2;

    !shuffled
}

fn load_map(map: &mut Vec<Vec<Location>>, contents: String) {
    use crate::Location::*;

    for line in contents.lines() {
        let mut row: Vec<Location> = Vec::new();

        for loc in line.chars() {
            match loc {
                '>' => row.push(East),
                'v' => row.push(South),
                '.' => row.push(Empty),
                _ => println!("Invalid location {} on line {}", loc, line),
            }
        }

        map.push(row);
    }
}

fn _print_map(map: &Vec<Vec<Location>>) {
    use crate::Location::*;

    println!("Map:");

    for row in map.iter() {
        for loc in row.iter() {
            match loc {
                Empty => print!("."),
                South => print!("v"),
                East => print!(">"),
            }
        }
        println!("");
    }

    println!("******");

    thread::sleep(time::Duration::from_secs(3));
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
