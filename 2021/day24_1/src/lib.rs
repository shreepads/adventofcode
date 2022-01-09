// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod alu;
mod alu2;

use std::fs;
use std::{thread, time};
use rand::prelude::*;


use alu::*;
use alu2::*;

pub fn calculate_max_serialno(file_path: &String) -> i64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut test_numbers = [[0i64; 14]; 10];
    let mut rng = thread_rng();
    

    // Generate 10 random serial numbers to test both alus
    for test_number in test_numbers.iter_mut() {
        for digit in test_number.iter_mut() {
            *digit = rng.gen_range(1..=9);
        }
    }

    // Setup alu
    let mut alu = Alu::new();

    for line in contents.lines() {
        alu.process_instruction(line.to_string());
    }

    for test_number in test_numbers {

        println!("Test number: {:?}", test_number);
        println!("*****************************");

        println!("Alu:");
        println!("--------");
        println!("w calc: {}", alu.calculate_var("w".to_string(), test_number));
        println!("x calc: {}", alu.calculate_var("x".to_string(), test_number));
        println!("y calc: {}", alu.calculate_var("y".to_string(), test_number));

        /*
        println!("z min max: {} {}", alu.var_mins.get("z").unwrap(), alu.var_maxs.get("z").unwrap());
        let min_val = alu.var_values.get("z").unwrap().iter().min().unwrap();
        let max_val = alu.var_values.get("z").unwrap().iter().max().unwrap();
        println!("z values range: {} - {}", min_val, max_val);
        */
        println!("z calc: {}", alu.calculate_var("z".to_string(), test_number));


        println!("Alu v2");
        println!("--------");
        let mut alu2 = Alu2::new();

        for line in contents.lines() {
            alu2.process_instruction(
                line.to_string(),
                test_number,
            );
        }

        println!("w calc: {}", alu2.vars.get("w").unwrap());
        println!("x calc: {}", alu2.vars.get("x").unwrap());
        println!("y calc: {}", alu2.vars.get("y").unwrap());
        println!("z calc: {}\n", alu2.vars.get("z").unwrap());

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
