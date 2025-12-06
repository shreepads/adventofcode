// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;
use std::fs;

pub fn num_dial_at_zero(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut password = 0;
    let mut position = 50;
    let dial_nums = 100;

    for instruction in file_contents.lines() {
        let direction = instruction.get(0..1).expect("Invalid instructon");
        let distance = make_mod(
            instruction
                .get(1..)
                .expect("Invalid instructon")
                .parse::<u64>()
                .unwrap(),
            dial_nums,
        );

        match direction {
            "R" => {
                position = add_mod(position, distance, dial_nums);
            }
            "L" => {
                position = sub_mod(position, distance, dial_nums);
            }
            _ => panic!("Invalid instruction"),
        }

        if position == 0 {
            password += 1
        };
    }

    password
}

// Convert num to mod base
#[inline(always)]
fn make_mod(num: u64, base: u64) -> u64 {
    num % base
}

// Add num2 to num1 mod base
// Works even if num1 num2 are not already mod base
#[inline(always)]
fn add_mod(num1: u64, num2: u64, base: u64) -> u64 {
    (num1 + num2) % base
}

// Subtract num2 from num1 mod base
// Assumes num1 and num2 are already mod base
#[inline(always)]
fn sub_mod(num1: u64, num2: u64, base: u64) -> u64 {
    match num1.cmp(&num2) {
        Ordering::Less => base - (num2 - num1),
        Ordering::Greater => num1 - num2,
        Ordering::Equal => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_mod() {
        let result = make_mod(9, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_add_mod() {
        let result = add_mod(9, 3, 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_sub_mod_gt() {
        let result = sub_mod(5, 2, 6);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sub_mod_lt() {
        let result = sub_mod(3, 5, 6);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub_mod_eq() {
        let result = sub_mod(5, 5, 6);
        assert_eq!(result, 0);
    }
}
