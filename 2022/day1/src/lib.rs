// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn most_cals_carried(file_path: &String) -> u64 {

    let list_of_lists_of_number = load_list_of_lists_of_unumbers(file_path);

    list_of_lists_of_number.iter()
        .map(|list_of_unumbers| list_of_unumbers.iter().sum())
        .max()
        .unwrap()
}

pub fn top3_total_cals_carried(file_path: &String) -> u64 {

    let list_of_lists_of_number = load_list_of_lists_of_unumbers(file_path);

    let mut list_of_total_cals = list_of_lists_of_number.iter()
        .map(|list_of_unumbers| list_of_unumbers.iter().sum())
        .collect::<Vec<u64>>();

    list_of_total_cals.sort_unstable();
    list_of_total_cals.reverse();

    list_of_total_cals
        .split_at(3).0
        .iter()
        .sum()

}

fn load_list_of_lists_of_unumbers(file_path: &String) -> Vec<Vec<u64>> {
    
    let mut list_of_lists_of_unumbers: Vec<Vec<u64>> = Vec::new();

    println!("Loading data from file:{}", file_path);
    let file_contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    for list_of_string_unumbers in file_contents.split("\n\n") {
        let list_of_unumbers: Vec<u64> = list_of_string_unumbers.lines()
            .map(|string_unumber| string_unumber.parse::<u64>().unwrap())
            .collect();

        list_of_lists_of_unumbers.push(list_of_unumbers);
    }

    list_of_lists_of_unumbers
}

