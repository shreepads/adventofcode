// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod alu;
//mod alu2;

use std::fs;
use std::{thread, time};


use alu::*;
//use alu2::*;

pub fn calculate_max_serialno(file_path: &String) -> i64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut alu = Alu::new();

    for (i, line) in contents.lines().enumerate() {
        alu.process_instruction(line.to_string());
        println!("*************************");
        println!("Line {}: {}", i, line);
        println!("*************************");
        //println!("ALU state: {:?} {:?}", alu.var_mins, alu.var_maxs);
        //println!("ALU state: {:?}", alu);
        for (var, vals) in alu.var_values.iter() {
            if vals.len() > 20 {
                println!("{}: {} - {}", var, vals.iter().min().unwrap(), vals.iter().max().unwrap());
            } else {
                println!("{}: {:?}", var, vals);
            }
        }
        //thread::sleep(time::Duration::from_secs(1));
    }

    println!("\n\n\n\n\n");
    println!("*************************");
    //println!("ALU z state: {:?}", alu.vars.get(&"z".to_string()).unwrap());

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
            calculate_max_serialno(&String::from("../resources/tests/day24-1-input-v1.txt"));
        assert_eq!(result, 1);
    }


}
