// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Copy)]
enum FoldDirection {
    Up, 
    Left,
    Invalid,
}

pub fn calculate_visible_dots(file_path: &String, folds: usize) -> usize {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut dot_posns : HashSet<(u16, u16)> = HashSet::new();
    let mut instructions: Vec<(FoldDirection, u16)> = Vec::new();

    load_dots_instructions(&mut dot_posns, &mut instructions, contents);

    for instruction in instructions.iter().take(folds) {
        fold(&mut dot_posns, *instruction);
    }

    dot_posns.len()
}

fn fold(dot_posns: &mut HashSet<(u16, u16)>, instruction: (FoldDirection, u16)) {

    let mut fold_dot_posns: Vec<(u16, u16)> = Vec::new();

    let (fold_dir, fold_posn) = instruction;

    for (x, y) in dot_posns.iter() {
        if fold_dir == FoldDirection::Up {
            if *y >= fold_posn {
                fold_dot_posns.push((*x, *y));
            }
        }

        if fold_dir == FoldDirection::Left {
            if *x >= fold_posn {
                fold_dot_posns.push((*x, *y));
            }
        }
    }

    for (x, y) in fold_dot_posns.iter() {
        
        dot_posns.remove( &(*x, *y) );

        if fold_dir == FoldDirection::Up {
            dot_posns.insert( (*x, fold_posn - (*y - fold_posn)) );
        }

        if fold_dir == FoldDirection::Left {
            dot_posns.insert( (fold_posn - (*x - fold_posn) , *y) );
        }
    }

}

fn load_dots_instructions(dot_posns: &mut HashSet<(u16, u16)>, 
    instructions: &mut Vec<(FoldDirection, u16)>, contents: String) {

    let mut load_instructions = false;

    for line in contents.lines() {
        
        // Single line divides points from instructions
        if line.trim().len() == 0 {
            load_instructions = true;
            continue;
        }

        if !load_instructions {
            let mut coords = line.split(",");
            let x = coords.next().unwrap().parse::<u16>().unwrap();
            let y = coords.next().unwrap().parse::<u16>().unwrap();
            dot_posns.insert((x, y));
        }

        if load_instructions {
            let mut instruction = line.split("=");
            let fold_dir = match instruction.next().unwrap().to_string().pop() {
                Some('x') => FoldDirection::Left,
                Some('y') => FoldDirection::Up,
                _         => {
                    println!("Invalid instruction: {}", line);
                    FoldDirection::Invalid
                },
            };

            let fold_point = instruction.next().unwrap().parse::<u16>().unwrap();
            instructions.push((fold_dir, fold_point));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day13_1_works() {

        let result =
            calculate_visible_dots(&String::from("../resources/tests/day13-1-testdata.txt"), 1);

        assert_eq!(result, 17);

    }
}
