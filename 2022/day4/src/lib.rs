// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn fully_contained_pairs(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let section_assignments = load_section_assignments(&file_contents);

    //println!("Section asignments: {:#?}", section_assignments);

    section_assignments.chunks(2)
        .filter(|assignment_pair| is_fully_contained(assignment_pair))
        .count()
}

fn load_section_assignments(file_contents: &String) -> Vec<(u64, u64)> {

    let mut section_assignments : Vec<(u64, u64)> = Vec::new();

    for line in file_contents.lines() {
        let mut assignments_iter = line.split(",");

        let first_assignment_str = assignments_iter.next().unwrap();
        let second_assignment_str = assignments_iter.next().unwrap();

        section_assignments.push(str_to_assignment(first_assignment_str));
        section_assignments.push(str_to_assignment(second_assignment_str));

    }

    section_assignments
}

fn str_to_assignment(assignment_str: &str) -> (u64, u64) {
    let mut section_id_iter = assignment_str.split("-");

    let start_section_id = section_id_iter.next().unwrap().parse::<u64>().unwrap();
    let end_section_id = section_id_iter.next().unwrap().parse::<u64>().unwrap();

    (start_section_id, end_section_id)
}

fn is_fully_contained(assignment_pair: &&[(u64, u64)]) -> bool {
    
    let mut assigment_pair_iter = assignment_pair.iter();

    let first_assignment: &(u64, u64) = assigment_pair_iter.next().unwrap();
    let second_assignment: &(u64, u64) = assigment_pair_iter.next().unwrap();
    
    let first_assignment_start: u64 = first_assignment.0;
    let first_assignment_end: u64 = first_assignment.1;

    let second_assignment_start: u64 = second_assignment.0;
    let second_assignment_end: u64 = second_assignment.1;

    if first_assignment_start == second_assignment_start {
        return true;
    }

    if first_assignment_end == second_assignment_end {
        return true;
    }

    if first_assignment_start < second_assignment_start {
        if second_assignment_end <= first_assignment_end {
            return true;
        }
    } else {
        if second_assignment_end >= first_assignment_end {
            return true;
        }
    }

    false
}