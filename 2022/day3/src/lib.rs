// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod rucksack;

use std::fs;

use rucksack::Rucksack;

pub fn total_priorities_of_common_types(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let rucksacks = load_rucksacks(&file_contents);

    rucksacks
        .iter()
        .map(|rucksack| rucksack.common_item_type_priority())
        .sum()
}

pub fn total_group_badge_priorities(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let rucksacks = load_rucksacks(&file_contents);

    rucksacks
        .chunks(3)
        .map(|group_rucksacks| Rucksack::common_group_item_type_priority(group_rucksacks))
        .sum()
}

fn load_rucksacks(file_contents: &String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in file_contents.lines() {
        rucksacks.push(Rucksack::new(line));
    }

    rucksacks
}
