// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;
use std::collections::HashMap;
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

    println!("{} dots: {:?}", dot_posns.len(), dot_posns);
    println!("{} instructions: {:?}", instructions.len(), instructions);

    for instruction in instructions.iter().take(folds) {
        fold(&mut dot_posns, *instruction);
    }

    dot_posns.len()
}

fn fold(dot_posns: &mut HashSet<(u16, u16)>, instruction: (FoldDirection, u16)) {

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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
