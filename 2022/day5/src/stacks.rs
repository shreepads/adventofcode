// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use regex::Regex;

const STACKS_COUNT: usize = 9;

#[derive(Debug)]
struct ProcedureStep {
    from_stack: usize,
    to_stack: usize,
    crates: usize,
}

impl ProcedureStep {
    pub fn new(procedure_step_str: &str) -> ProcedureStep {
        ProcedureStep {
            from_stack: 1,
            to_stack: 2,
            crates: 1,
        }
    }
}


#[derive(Debug)]
pub struct Stacks {
    stacks: [Vec<char>; STACKS_COUNT],
    procedure_step_regex: Regex,          // hold here to avoid recompilation
}

impl Stacks {
    pub fn new(init_stack_str: &str) -> Stacks {
        // Obviously can't be too easy
        let mut stacks = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new() ];

        let mut init_stack_str_iter = init_stack_str.lines().rev();

        // Discard the bottom line of numbers
        _ = init_stack_str_iter.next().unwrap();

        for stack_line in init_stack_str_iter {
            // Parse the line and load appropriate stacks
            // Some might be empty
            for stack_index in 0..STACKS_COUNT {
                let crate_index = (4 * stack_index) + 1;
                let crate_char = stack_line.chars().nth(crate_index).unwrap();
                if crate_char != ' ' {
                    // insert crate char in given stack
                    stacks[stack_index].push(crate_char);
                }
            }
        }

        // init here to avoid recompilation
        let procedure_step_regex = Regex::new(r"^move (?P<crates>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();

        Stacks {
            stacks,
            procedure_step_regex,
        }
    }


    pub fn execute_procedure_step(&self, procedure_step_str: &str) {
        let procedure_step_caps = self.procedure_step_regex.captures(procedure_step_str).unwrap();

        let from_stack = procedure_step_caps["from"].parse::<usize>().unwrap();
        let to_stack = procedure_step_caps["to"].parse::<usize>().unwrap();
        let crates = procedure_step_caps["crates"].parse::<usize>().unwrap();

        //println!("Move {} from {} to {}", crates, from_stack, to_stack);

        
    }
}