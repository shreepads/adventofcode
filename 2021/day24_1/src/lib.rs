// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod alu;
mod alu2;
mod ula;

use std::fs;
use std::{thread, time};
use std::collections::HashMap;
use std::collections::HashSet;

use rand::prelude::*;

use alu::*;
use alu2::*;
use ula::Ula;

pub fn calculate_max_serialno(file_path: &String) -> i64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    // Setup alu
    let mut alu = Alu::new();
    let mut alu_vals : Vec<HashMap<String, HashSet<i64>>> = Vec::new();

    for line in contents.lines() {
        alu.process_instruction(line.to_string());
        alu_vals.push(alu.var_values.clone());
    }

    // Setp ula
    let mut ula = Ula::new();
    //println!("Ula state: {:?}", ula);
    let mut alu_vals_rev = alu_vals.iter().rev();
    // skip the last state vals in alu_vals
    alu_vals_rev.next();
    for line in contents.lines().rev() {
        
        println!("Ula state: {:?}", ula);
        
        ula.process_instruction(line.to_string(), alu_vals_rev.next().unwrap());
        
        println!("");
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;
/*
    #[test]
    fn tiny_add() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata1.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn tiny_add_mul() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata2.txt"));
        assert_eq!(result, 5);
    }

    #[test]
    fn tiny_add_mul_div() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata3.txt"));
        assert_eq!(result, 4);
    }

    #[test]
    fn tiny_add_mul_div_mod() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata4.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn tiny_add_commutative() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata5.txt"));
        assert_eq!(result, 2);
    }

    #[test]
    fn tiny_mul_commutative() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata6.txt"));
        assert_eq!(result, 2);
    }

    #[test]
    fn tiny_eql() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata7.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn calc_z() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata8.txt"));
        assert_eq!(result, 1);
    }
*/

    #[test]
    fn test_prod() {
        let result =
            calculate_max_serialno(&String::from("../resources/day24-1-input.txt"));
        assert_eq!(result, 1);
    }


}
