// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn total_joltage(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut total_j = 0;

    for bank in file_contents.lines() {
        total_j += max_joltage(bank);
    }

    total_j
}

fn max_joltage(bank: &str) -> u64 {
    let mut left_bat_pos = 0;
    let mut left_bat_volt = 0;

    let mut right_bat_volt = 0;

    for (i, c) in bank.chars().enumerate() {
        // Don't consider the last bat
        if i == bank.len() - 1 {
            break;
        };

        let v = c.to_digit(10).unwrap() as u64;

        if v > left_bat_volt {
            left_bat_pos = i;
            left_bat_volt = v;
        }
    }

    for (_, c) in bank.chars().enumerate().skip(left_bat_pos + 1) {
        let v = c.to_digit(10).unwrap() as u64;

        if v > right_bat_volt {
            right_bat_volt = v;
        }
    }

    (left_bat_volt * 10) + right_bat_volt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage1() {
        let result = max_joltage("987654321111111");
        assert_eq!(result, 98);
    }

    #[test]
    fn test_max_joltage2() {
        let result = max_joltage("811111111111119");
        assert_eq!(result, 89);
    }

    #[test]
    fn test_max_joltage3() {
        let result = max_joltage("234234234234278");
        assert_eq!(result, 78);
    }

    #[test]
    fn test_max_joltage4() {
        let result = max_joltage("818181911112111");
        assert_eq!(result, 92);
    }
}
