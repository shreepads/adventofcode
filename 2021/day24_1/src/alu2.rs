// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub struct Alu2 {
    pub vars: HashMap<String, i64>,
    current_input: usize,
}

impl Alu2 {
    pub fn new() -> Alu2 {

        Alu2 {
            vars: HashMap::from([
                ("w".to_string(), 0),
                ("x".to_string(), 0),
                ("y".to_string(), 0),
                ("z".to_string(), 0),
            ]),
            current_input: 0,
        }
    }

    pub fn process_instruction(&mut self, instr: String, input: [i64; 14]) {

        let mut parts = instr.split(' ');

        let op = parts.next().unwrap();

        let left = parts.next().unwrap();
        if left.len() > 1  ||  !"wxyz".contains(left) {
            println!("Invalid left in instr: {}", instr);
        }

        match op {
            "inp" => self.process_inp(left, input),
            "add" => {
                let right = parts.next().unwrap();
                self.process_add(left, self.right_val(right));
            },
            "mul" => {
                let right = parts.next().unwrap();
                self.process_mul(left, self.right_val(right));
            },
            "div" => {
                let right = parts.next().unwrap();
                self.process_div(left, self.right_val(right));
            },
            "mod" => {
                let right = parts.next().unwrap();
                self.process_mod(left, self.right_val(right));
            },
            "eql" => {
                let right = parts.next().unwrap();
                self.process_eql(left, self.right_val(right));
            },
            _     => {
                println!("Invalid instruction: {}", instr);
            }
        }
    }

    fn right_val(&self, right: &str) -> i64 {
        // figure out if right is a register or value
        match right.parse::<i64>() {
            Ok(num) => num,
            Err(_)  => *self.vars.get(right).unwrap(),
        }

    }

    fn process_inp(&mut self, left: &str, input: [i64; 14]) {
        self.vars.insert(left.to_string(), input[self.current_input]);
        self.current_input += 1;
    }

    fn process_add(&mut self, left: &str, right_val: i64) {
        let left_val  = self.vars.get(left).unwrap().clone();
        self.vars.insert(left.to_string(), left_val + right_val);
    }

    fn process_mul(&mut self, left: &str, right_val: i64) {
        let left_val  = self.vars.get(left).unwrap().clone();
        self.vars.insert(left.to_string(), left_val * right_val);
    }

    fn process_div(&mut self, left: &str, right_val: i64) {
        let left_val  = self.vars.get(left).unwrap().clone();
        self.vars.insert(left.to_string(), left_val / right_val);
    }

    fn process_mod(&mut self, left: &str, right_val: i64) {
        let left_val  = self.vars.get(left).unwrap().clone();
        self.vars.insert(left.to_string(), left_val % right_val);
    }

    fn process_eql(&mut self, left: &str, right_val: i64) {
        let left_val  = self.vars.get(left).unwrap().clone();
        self.vars.insert(left.to_string(), 
            if left_val == right_val {1} else {0}
        );
    }

}