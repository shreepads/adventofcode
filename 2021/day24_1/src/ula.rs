// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

use crate::alu::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UlaValue {
    Unknown(HashSet<i64>),
    Known(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ula {
    pub vars: HashMap<String, UlaValue>,
    current_input: usize,
}

impl Ula {
    pub fn new() -> Ula {
        use crate::ula::UlaValue::{Known, Unknown};

        Ula {
            vars: HashMap::from([
                ("w".to_string(), Unknown(HashSet::new())),
                ("x".to_string(), Unknown(HashSet::new())),
                ("y".to_string(), Unknown(HashSet::new())),
                ("z".to_string(), Known(0)),
            ]),
            current_input: 0,
        }
    }

    pub fn process_instruction(&mut self, instr: String, alu_vals: &HashMap<String, HashSet<i64>>) {

        println!("Processing instr {} with vals:", instr);
        for (var, vals) in alu_vals.iter() {
            print!("{}: ", var);
            if vals.len() > 26 {
                println!("{} - {}", vals.iter().min().unwrap(), vals.iter().max().unwrap());
            } else {
                println!("{:?}", vals.iter());
            }
        }
        
        let mut parts = instr.split(' ');

        let op = parts.next().unwrap();

        let left = parts.next().unwrap();
        if left.len() > 1 || !"wxyz".contains(left) {
            println!("Invalid left in instr: {}", instr);
        }

        match op {
            //"inp" => self.process_inp(left),
            "add" => {
                let right = parts.next().unwrap();
                self.process_add(left, right, self.right_values(right, alu_vals), alu_vals);
            },
            "mul" => {
                let right = parts.next().unwrap();
                self.process_mul(left, right, self.right_values(right, alu_vals), alu_vals);
            },
            "eql" => {
                let right = parts.next().unwrap();
                self.process_eql(left, right, self.right_values(right, alu_vals), alu_vals);
            },
            _ => {
                println!("Invalid instruction: {}", instr);
            },
            /*
            "div" => {
                let right = parts.next().unwrap();
                self.process_div(left, self.right_val(right),
                    self.right_min(right), self.right_max(right), self.right_values(right));
            }
            "mod" => {
                let right = parts.next().unwrap();
                self.process_mod(left, self.right_val(right),
                    self.right_min(right), self.right_max(right), self.right_values(right));
            }
            _ => {
                println!("Invalid instruction: {}", instr);
            }*/
        }
    }


    fn right_values(&self, right: &str, alu_vals: &HashMap<String, HashSet<i64>>) -> HashSet<i64> {

        use crate::ula::UlaValue::*;

        // figure out if right is a register or value
        match right.parse::<i64>() {
            Ok(num) => HashSet::from([num]),
            Err(_) => {
                // check if register has known value else use alu_vals
                if let Known(right_v) = self.vars.get(right).unwrap() {
                    HashSet::from([*right_v])
                } else {
                    alu_vals.get(right).unwrap().clone()
                }
            },
        }
    }


    fn process_add(&mut self, left: &str, right: &str, right_values: HashSet<i64>, alu_vals: &HashMap<String, HashSet<i64>>) {

        use crate::ula::UlaValue::*;

        let left_values: HashSet<i64> = alu_vals.get(left).unwrap().clone();

        let left_val: UlaValue = self.vars.get(left).unwrap().clone();
        //let right_val: UlaValue = self.vars.get(right).unwrap().clone();

        // If left value is known, check left/ right values for possible solutions
        let mut new_left_values : HashSet<i64> = HashSet::new();
        let mut new_right_values : HashSet<i64> = HashSet::new();

        if let Known(left_v) = left_val {
            for left_value in left_values.iter() {
                for right_value in right_values.iter() {
                    if left_value + right_value == left_v {
                        new_left_values.insert(*left_value);
                        new_right_values.insert(*right_value);
                    }
                }
            }
        }

        if new_left_values.len() == 1 {
            self.vars.insert(left.to_string(), Known(*new_left_values.iter().nth(0).unwrap()));
        } else {
            self.vars.insert(left.to_string(), Unknown(new_left_values));
        }

        if "wxyz".contains(right) {
            if new_right_values.len() == 1 {
                self.vars.insert(right.to_string(), Known(*new_right_values.iter().nth(0).unwrap()));
            } else if new_right_values.len() > 1 {
                self.vars.insert(right.to_string(), Unknown(new_right_values));
            }
        }
    }


    fn process_mul(&mut self, left: &str, right: &str, right_values: HashSet<i64>, alu_vals: &HashMap<String, HashSet<i64>>) {

        use crate::ula::UlaValue::*;

        let left_values: HashSet<i64> = alu_vals.get(left).unwrap().clone();

        let left_val: UlaValue = self.vars.get(left).unwrap().clone();
        //let right_val: UlaValue = self.vars.get(right).unwrap().clone();

        // If left value is known, check left/ right values for possible solutions
        let mut new_left_values : HashSet<i64> = HashSet::new();
        let mut new_right_values : HashSet<i64> = HashSet::new();

        if let Known(left_v) = left_val {
            for left_value in left_values.iter() {
                for right_value in right_values.iter() {
                    if left_value * right_value == left_v {
                        new_left_values.insert(*left_value);
                        new_right_values.insert(*right_value);
                    }
                }
            }
        }

        if new_left_values.len() == 1 {
            self.vars.insert(left.to_string(), Known(*new_left_values.iter().nth(0).unwrap()));
        } else {
            self.vars.insert(left.to_string(), Unknown(new_left_values));
        }

        if "wxyz".contains(right) {
            if new_right_values.len() == 1 {
                self.vars.insert(right.to_string(), Known(*new_right_values.iter().nth(0).unwrap()));
            } else  if new_right_values.len() > 1 {
                self.vars.insert(right.to_string(), Unknown(new_right_values));
            }
        }
    }


    fn process_eql(&mut self, left: &str, right: &str, right_values: HashSet<i64>, alu_vals: &HashMap<String, HashSet<i64>>) {

        use crate::ula::UlaValue::*;

        let left_values: HashSet<i64> = alu_vals.get(left).unwrap().clone();

        let left_val: UlaValue = self.vars.get(left).unwrap().clone();

    }
    
}
