// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn calculate_outputs_total(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut signal_patterns: Vec<HashMap<usize, HashSet<char>>> = Vec::new();
    let mut output_patterns: Vec<Vec<HashSet<char>>> = Vec::new();

    load_data(&mut signal_patterns, &mut output_patterns, contents);

    let mut outputs_total = 0u32;

    for i in 0..signal_patterns.len() {
        outputs_total += find_output(&mut signal_patterns[i], &output_patterns[i]);
    }

    outputs_total
}

fn load_data(
    signal_patterns: &mut Vec<HashMap<usize, HashSet<char>>>,
    output_patterns: &mut Vec<Vec<HashSet<char>>>,
    contents: String,
) {
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

fn find_output(
    signal_pattern: &mut HashMap<usize, HashSet<char>>,
    output_pattern: &Vec<HashSet<char>>,
) -> u32 {
    // Deterministic logic to find the digit pattern match

    let mut digit_pattern_map: HashMap<u32, &HashSet<char>> = HashMap::new();
    let mut removal_keys: Vec<usize> = Vec::new();

    // Find 1, 4, 7, 8 by length

    let signal_pattern_clone1 = signal_pattern.clone(); // mut immut borrower check

    for (key, signal) in signal_pattern_clone1.iter() {
        match signal.len() {
            2 => {
                digit_pattern_map.insert(1, signal);
                removal_keys.push(*key);
            }
            4 => {
                digit_pattern_map.insert(4, signal);
                removal_keys.push(*key);
            }
            3 => {
                digit_pattern_map.insert(7, signal);
                removal_keys.push(*key);
            }
            7 => {
                digit_pattern_map.insert(8, signal);
                removal_keys.push(*key);
            }
            _ => {}
        }
    }

    for key in removal_keys.iter() {
        // drain doesn't work
        signal_pattern.remove(key);
    }

    // Find 3, 9, 6 by comparing with 1, 4, 7 respectively

    let signal_pattern_clone2 = signal_pattern.clone(); // mut immut borrower check

    for (key, signal) in signal_pattern_clone2.iter() {
        match signal.len() {
            5 => {
                // 3 superset of 1
                if signal.is_superset(digit_pattern_map.get(&1).unwrap()) {
                    digit_pattern_map.insert(3, signal);
                    removal_keys.push(*key);
                }
            }
            6 => {
                // 9, 6 compared to 4, 7 respectively

                // 9 superset of 4
                if signal.is_superset(digit_pattern_map.get(&4).unwrap()) {
                    digit_pattern_map.insert(9, signal);
                    removal_keys.push(*key);
                }
                // 6 not superset of 7
                if !signal.is_superset(digit_pattern_map.get(&7).unwrap()) {
                    digit_pattern_map.insert(6, signal);
                    removal_keys.push(*key);
                }
            }
            _ => {}
        }
    }

    for key in removal_keys.iter() {
        // drain doesn't work
        signal_pattern.remove(key);
    }

    // Find 0 by length, 2, 5 by comparing with 6 respectively

    let signal_pattern_clone3 = signal_pattern.clone(); // mut immut borrower check

    for (key, signal) in signal_pattern_clone3.iter() {
        match signal.len() {
            6 => {
                digit_pattern_map.insert(0, signal);
                removal_keys.push(*key);
            }
            5 => {
                // 2, 5 compared with 6

                // 5 subset of 6
                if signal.is_subset(digit_pattern_map.get(&6).unwrap()) {
                    digit_pattern_map.insert(5, signal);
                    removal_keys.push(*key);
                }
                // 2 not subset of 6
                if !signal.is_subset(digit_pattern_map.get(&6).unwrap()) {
                    digit_pattern_map.insert(2, signal);
                    removal_keys.push(*key);
                }
            }
            _ => {}
        }
    }

    output(output_pattern, &digit_pattern_map)
}

fn output(
    output_pattern: &Vec<HashSet<char>>,
    digit_pattern_map: &HashMap<u32, &HashSet<char>>,
) -> u32 {
    let mut output_val = 0u32;

    for (i, output) in output_pattern.iter().enumerate() {
        let digit = match output.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => {
                let (d, _) = digit_pattern_map
                    .iter()
                    .find(|(_, v)| **v == output)
                    .unwrap();
                *d
            }
        };

        output_val += digit * 10u32.pow((3 - i).try_into().unwrap());
    }

    output_val
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
