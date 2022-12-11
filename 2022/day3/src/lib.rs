// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

const MAX_PRIORITY : usize = 52;

#[derive(Debug)]
struct Rucksack {
    contents: [(u64, u64); MAX_PRIORITY],
}

impl Rucksack {
    pub fn new(line: &str) -> Rucksack {

        let mut contents: [(u64, u64); MAX_PRIORITY] = [(0, 0); MAX_PRIORITY];

        let (first_compartment, second_compartment) = line.split_at(line.len()/2);

        for ch in first_compartment.chars() {
            let priority_index = Self::get_priority_from_char(ch) - 1;
            contents[priority_index].0 += 1;
        }

        for ch in second_compartment.chars() {
            let priority_index = Self::get_priority_from_char(ch) - 1;
            contents[priority_index].1 += 1;
        }

        Rucksack {
            contents,
        }
    }

    #[inline(always)]
    fn get_priority_from_char(c: char) -> usize {
        // convert a-z to 1-26
        // convert A-Z to 27-52

        let priority = match c {
            'a'..='z' => (c as u32) - 97 + 1,
            'A'..='Z' => (c as u32) - 65 + 27,
            _        => 0,
        };

        priority as usize
    } 
}


pub fn total_priorities_of_common_types(file_path: &String) -> u64 {

    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let rucksacks = load_rucksacks(&file_contents);

    println!("Rucksacks: {:#?}", rucksacks);

    2

}

fn load_rucksacks(file_contents: &String) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in file_contents.lines() {
        rucksacks.push(Rucksack::new(line));
    }
    rucksacks
}
