// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod alu;

use std::fs;

use alu::*;

pub fn calculate_max_serialno(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut alu = Alu::new();

    for line in contents.lines() {
        alu.process_instruction(line.to_string());
    }

    println!("ALU state: {:#?}", alu);
    //println!("ALU z state: {:?}", alu.vars.get(&"z".to_string()).unwrap());

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tiny_add() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata1.txt"));
        assert_eq!(result, 0);
    }

    #[test]
    fn tiny_add_mul() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata2.txt"));
        assert_eq!(result, 0);
    }

    #[test]
    fn tiny_add_mul_div() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata3.txt"));
        assert_eq!(result, 0);
    }


    #[test]
    fn tiny_add_mul_div_mod() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata4.txt"));
        assert_eq!(result, 0);
    }


    #[test]
    fn tiny_add_commutative() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata5.txt"));
        assert_eq!(result, 0);
    }


    #[test]
    fn tiny_mul_commutative() {
        let result = calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata6.txt"));
        assert_eq!(result, 4);
    }


}
