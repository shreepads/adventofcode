// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod stacks;

use stacks::Stacks;
use std::fs;

pub fn rearrange_message(file_path: &String) -> String {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut file_contents_iter = file_contents.split("\n\n");

    let init_stack_str = file_contents_iter.next().unwrap();
    let procedure_str = file_contents_iter.next().unwrap();

    // Create the init stacks
    let mut stacks = Stacks::new(init_stack_str);

    //println!("Init stack: {:#?}", stacks);

    for procedure_step_str in procedure_str.lines() {
        stacks.execute_procedure_step(procedure_step_str);
    }

    println!("Final stack: {:?}", stacks);

    String::from("MESSAGE")
}

pub fn correct_rearrange_message(file_path: &String) -> String {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut file_contents_iter = file_contents.split("\n\n");

    let init_stack_str = file_contents_iter.next().unwrap();
    let procedure_str = file_contents_iter.next().unwrap();

    // Create the init stacks
    let mut stacks = Stacks::new(init_stack_str);

    //println!("Init stack: {:#?}", stacks);

    for procedure_step_str in procedure_str.lines() {
        stacks.execute_correct_procedure_step(procedure_step_str);
    }

    println!("Final stack: {:?}", stacks);

    String::from("MESSAGE")
}
