// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs;

pub const MAX_BRUTE_STEPS: usize = 14;

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

    let mut element_counts = element_counts(&polymer);

    // Setup pair counts
    let mut polypair_map : HashMap<String, u64> = HashMap::new();

    for (_, pair) in polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
    {
        let pair_count = polypair_map.entry(pair.iter().collect::<String>()).or_insert(0);
        *pair_count += 1;
    }

    println!("Setup completed: polymer len {}, element counts {:?}, polypair map {:?}",
        polymer.len(), 
        element_counts,
        polypair_map
    );

    let mut newpair_map : HashMap<String, u64> = HashMap::new();

    for i in 1..=steps {

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

        println!("Round {}: Adding {} new pair counts", i, newpair_map.len());

        for (k, v) in newpair_map.drain() {
            polypair_map.insert(k, v);
        }
    }


    let max_count = element_counts.values().max().unwrap();
    let min_count = element_counts.values().min().unwrap();

    max_count - min_count
}

fn hashmap_polymerise_count(pair: String, pair_insert_map: &HashMap<String, char>,
    element_counts: &mut HashMap<char, u64>, steps: usize) {

    let mut polypair_map : HashMap<String, u64> = HashMap::new();
    let mut newpair_map : HashMap<String, u64> = HashMap::new();

    polypair_map.insert(pair, 1);

    for i in 1..=steps {
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

        //println!("Round {}: Adding {} new pair counts", i, newpair_map.len());

        for (k, v) in newpair_map.drain() {
            polypair_map.insert(k, v);
        }
    }
    
}

fn list_polymerise_count(pair: String, pair_insert_map: &HashMap<String, char>,
    element_counts: &mut HashMap<char, u64>, steps: usize) {

    let mut polypair_list : Vec<String> = Vec::with_capacity(70000000);
    let mut newpair_list : Vec<String> = Vec::with_capacity(70000000);

    polypair_list.push(pair);

    for i in 1..=steps {
        while let Some(polypair) = polypair_list.pop() {

            let first_element: char = polypair.chars().nth(0).unwrap();
            let second_element: char = polypair.chars().nth(1).unwrap();

            // get new element for pair
            let new_element: char = *pair_insert_map.get(&polypair).unwrap();

            // increment new element count
            let count = element_counts.entry(new_element).or_insert(0);
            *count += 1;

            // add new pairs
            let first_pair = format!("{}{}", first_element, new_element);
            newpair_list.push(first_pair);
            let second_pair = format!("{}{}", new_element, second_element);
            newpair_list.push(second_pair);

        }

        println!("Round {}: Adding {} new pairs", i, newpair_list.len());

        // also clears out newpair_list - sequence doesn't matter
        polypair_list.append(&mut newpair_list);

    }

}

fn tree_polymerise_count(pair: String, pair_insert_map: &HashMap<String, char>,
    element_counts: &mut HashMap<char, u64>, steps: usize) {

    let mut poly_tree : BTreeMap<u64, char> = BTreeMap::new();

    let first: char = pair.chars().nth(0).unwrap();
    let second: char = pair.chars().nth(1).unwrap();

    poly_tree.insert(0, first);
    poly_tree.insert(u64::MAX, second);

    //println!("Starting poly_tree with pair {}", pair);

    let mut new_elements: Vec<(u64, char)> = Vec::with_capacity(70000000);

    for i in 1..=steps {
        
        let mut prev_idx: u64 = 0;
        let mut prev_element: char = first;

        // move allocation out
        //let mut new_elements: Vec<(u64, char)> = Vec::new();

        // find new element insertion points
        for (idx, element) in poly_tree.iter() {
            // no action on first element
            if *idx == 0 {
                continue;
            }

            // lookup element pair
            let mut pair_str = String::new();
            pair_str.push(prev_element);
            pair_str.push(*element);

            let new_element: char = *pair_insert_map.get(&pair_str).unwrap();
            let new_element_idx: u64 = prev_idx/2 + idx/2;

            new_elements.push((new_element_idx, new_element));

            prev_idx = *idx;
            prev_element = *element;
        }

        // add new elements to tree and counter
        for (new_element_idx, new_element) in new_elements.iter() {

            poly_tree.insert(*new_element_idx, *new_element);
            
            let count = element_counts.entry(*new_element).or_insert(0);
            *count += 1;
        }

        //println!("Tree round {}: Inserted elements {:?}", i, new_elements);
        new_elements.clear();
    }
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
        let result = calculate_element_diff(
            &String::from("../resources/tests/day14-2-testdata.txt"), 
            40);
            
        assert_eq!(result, 2188189693529);
    }
}
