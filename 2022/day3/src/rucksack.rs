// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

const MAX_PRIORITY: usize = 52;

#[derive(Debug)]
pub struct Rucksack {
    contents: [(u64, u64); MAX_PRIORITY],
}

impl Rucksack {
    pub fn new(line: &str) -> Rucksack {
        let mut contents: [(u64, u64); MAX_PRIORITY] = [(0, 0); MAX_PRIORITY];

        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

        for ch in first_compartment.chars() {
            let priority_index = Self::get_priority_from_char(ch) - 1;
            contents[priority_index].0 += 1;
        }

        for ch in second_compartment.chars() {
            let priority_index = Self::get_priority_from_char(ch) - 1;
            contents[priority_index].1 += 1;
        }

        Rucksack { contents }
    }

    pub fn common_item_type_priority(&self) -> usize {
        for (priority_index, compartment_counts) in self.contents.iter().enumerate() {
            let (first_compartment_count, second_compartment_count) = compartment_counts;
            if *first_compartment_count > 0 && *second_compartment_count > 0 {
                return priority_index + 1;
            }
        }

        0
    }

    #[inline(always)]
    fn get_priority_from_char(c: char) -> usize {
        // convert a-z to 1-26
        // convert A-Z to 27-52

        let priority = match c {
            'a'..='z' => (c as u32) - 97 + 1,
            'A'..='Z' => (c as u32) - 65 + 27,
            _ => 0,
        };

        priority as usize
    }
}
