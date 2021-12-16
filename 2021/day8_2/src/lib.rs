// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn calculate_outputs_total(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut signal_patterns: Vec<HashMap<usize, HashSet<char>>> = Vec::new();
    let mut output_patterns: Vec<Vec<HashSet<char>>> = Vec::new();

    load_data(&mut signal_patterns, &mut output_patterns, contents);
        
    println!("Loaded {} signal patterns and {} output patterns",
        signal_patterns.len(), output_patterns.len()
    );

    println!("First signal: {:?}", signal_patterns[0]);
    println!("First output: {:?}", output_patterns[0]);

    let mut outputs_total = 0u32;

    for i in 0..signal_patterns.len() {
        outputs_total = find_output(&mut signal_patterns[i], &output_patterns[i]);
    }

    outputs_total
}

fn load_data(signal_patterns: &mut Vec<HashMap<usize, HashSet<char>>>,
    output_patterns: &mut Vec<Vec<HashSet<char>>>, contents: String) {

    for line in contents.lines() {

        let mut signal_pattern: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut output_pattern: Vec<HashSet<char>> = Vec::new();

        for (i, line_part) in line.split(" | ").enumerate() {
            if i == 0 {
                for (j, signal_str) in line_part.split(" ").enumerate() {
                    let signal: HashSet<char> = signal_str.chars().collect(); 
                    signal_pattern.insert(j, signal);
                }
                continue;
            }

            for output_str in line_part.split(" ") {
                let output: HashSet<char> = output_str.chars().collect(); 
                output_pattern.push(output);
            }
        }

        signal_patterns.push(signal_pattern);
        output_patterns.push(output_pattern);

    }
        
}

fn find_output(signal_pattern: &mut HashMap<usize, HashSet<char>>,
    output_pattern: &Vec<HashSet<char>>) -> u32 {

    println!("Finding ouput with signal pattern {:?} and output pattern {:?}", signal_pattern, output_pattern);

    0u32
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
