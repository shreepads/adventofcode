// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod alu;
mod alu2;

use std::fs;

use alu::*;
use alu2::*;

pub fn calculate_max_serialno(file_path: &String) -> i64 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut alu = Alu2::new();
    let input: [i64; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5];

    for line in contents.lines() {
        alu.process_instruction(line.to_string(), input);
    }

    //println!("ALU state: {:#?}", alu);
    //println!("ALU z state: {:?}", alu.vars.get(&"z".to_string()).unwrap());
    println!("ALU z value: {}", alu.vars.get("z").unwrap());


    //alu.calculate_z(input) as u32
    *alu.vars.get("z").unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tiny_add() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata1.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn tiny_add_mul() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata2.txt"));
        assert_eq!(result, 5);
    }

    #[test]
    fn tiny_add_mul_div() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata3.txt"));
        assert_eq!(result, 4);
    }


    #[test]
    fn tiny_add_mul_div_mod() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata4.txt"));
        assert_eq!(result, 1);
    }


    #[test]
    fn tiny_add_commutative() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata5.txt"));
        assert_eq!(result, 2);
    }


    #[test]
    fn tiny_mul_commutative() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata6.txt"));
        assert_eq!(result, 2);
    }


    #[test]
    fn tiny_eql() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata7.txt"));
        assert_eq!(result, 1);
    }


    #[test]
    fn calc_z() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata8.txt"));
        assert_eq!(result, 1);
    }
}
