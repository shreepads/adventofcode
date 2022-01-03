// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;

pub const MAX_BRUTE_STEPS: usize = 14;

pub fn calculate_element_diff(file_path: &String, steps: usize) -> u64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut polymer: String = String::new();

    let mut pair_insert_map: HashMap<String, char> = HashMap::new();

    load_data(&mut polymer, &mut pair_insert_map, contents);

    let mut element_counts = element_counts(&polymer);

    // Setup pair counts
    let mut polypair_map: HashMap<String, u64> = HashMap::new();

    for (_, pair) in polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
    {
        let pair_count = polypair_map
            .entry(pair.iter().collect::<String>())
            .or_insert(0);
        *pair_count += 1;
    }

    let mut newpair_map: HashMap<String, u64> = HashMap::new();

    for _ in 1..=steps {
        for (polypair, paircount) in polypair_map.drain() {
            let first_element: char = polypair.chars().nth(0).unwrap();
            let second_element: char = polypair.chars().nth(1).unwrap();

            // get new element for pair
            let new_element: char = *pair_insert_map.get(&polypair).unwrap();

            // increment new element count
            let count = element_counts.entry(new_element).or_insert(0);
            *count += paircount;

            // add new pairs
            let first_pair = format!("{}{}", first_element, new_element);
            let first_pair_count = newpair_map.entry(first_pair).or_insert(0);
            *first_pair_count += paircount;

            let second_pair = format!("{}{}", new_element, second_element);
            let second_pair_count = newpair_map.entry(second_pair).or_insert(0);
            *second_pair_count += paircount;
        }

        for (k, v) in newpair_map.drain() {
            polypair_map.insert(k, v);
        }
    }

    let max_count = element_counts.values().max().unwrap();
    let min_count = element_counts.values().min().unwrap();

    max_count - min_count
}

fn element_counts(polymer: &String) -> HashMap<char, u64> {
    let mut element_counts: HashMap<char, u64> = HashMap::new();

    for element in polymer.chars() {
        if !element_counts.contains_key(&element) {
            element_counts.insert(element, 0);
        }

        let element_count = element_counts.get_mut(&element).unwrap();

        *element_count += 1;
    }

    element_counts
}

fn load_data(polymer: &mut String, pair_insert_map: &mut HashMap<String, char>, contents: String) {
    for (line_no, line) in contents.lines().enumerate() {
        if line_no == 0 {
            polymer.push_str(line);
            continue;
        }

        if line_no == 1 {
            continue;
        }

        let mut rule = line.split(" -> ");

        let pair: String = rule.next().unwrap().to_string();
        let ins_element: char = rule.next().unwrap().chars().nth(0).unwrap();

        pair_insert_map.insert(pair, ins_element);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn poly_tree() {
        let result =
            calculate_element_diff(&String::from("../resources/tests/day14-2-testdata.txt"), 40);

        assert_eq!(result, 2188189693529);
    }
}
