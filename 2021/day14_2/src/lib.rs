// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;

pub const MAX_BRUTE_STEPS: usize = 16;

pub fn calculate_element_diff(file_path: &String, steps: usize) -> u64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut polymer: String = String::with_capacity(10000);

    let mut pair_insert_map: HashMap<String, char> = HashMap::new();

    load_data(&mut polymer, &mut pair_insert_map, contents);

    //println!("{} pair rules: {:?}", pair_insert_map.len(), pair_insert_map);

    for i in 1..=MAX_BRUTE_STEPS {
        polymerise(&mut polymer, &pair_insert_map);
    }

    let mut element_counts = element_counts(&polymer);

    println!("Completed brute force stage, got polymer len {}, counts {:?}", polymer.len(), element_counts);

    for (i, pair) in polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
    {
        tree_polymerise_count(pair.iter().collect::<String>(), &pair_insert_map, &mut element_counts, steps - MAX_BRUTE_STEPS);
    }


    0
}

fn tree_polymerise_count(pair: String, pair_insert_map: &HashMap<String, char>,
    element_counts: &mut HashMap<char, u64>, steps: usize) {

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

fn polymerise(polymer: &mut String, pair_insert_map: &HashMap<String, char>) {

    let mut inserts: Vec<(usize, char)> = Vec::new();

    for (i, pair) in polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
    {
        let pair_str = pair.iter().collect::<String>();

        inserts.push((2 * i + 1, *pair_insert_map.get(&pair_str).unwrap()));
    }

    for (posn, insert_element) in inserts {
        polymer.insert(posn, insert_element);
    }
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
        let result = calculate_element_diff(&String::from("../resources/tests/day14-2-testdata.txt"), 40);
        assert_eq!(result, 2188189693529);
    }
}
