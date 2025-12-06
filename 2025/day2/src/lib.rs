// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn sum_invalid_ids(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut sum = 0;

    for id_range in file_contents.split(",") {
        let mut id_iter = id_range.split("-");

        let start_id = id_iter.next().unwrap().parse::<u64>().unwrap();
        let end_id = id_iter.next().unwrap().parse::<u64>().unwrap();

        sum += (start_id..end_id + 1)
            .filter(|x| !is_valid_id(*x))
            .fold(0, |acc, x| acc + x);
    }

    sum
}

fn is_valid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let id_str_len = id_str.len();

    if id_str_len % 2 == 1 {
        return true;
    }

    let num1 = id_str
        .get(0..id_str_len / 2)
        .expect("Unexpected error")
        .parse::<u64>()
        .unwrap();

    let num2 = id_str
        .get(id_str_len / 2..)
        .expect("Unexpected error")
        .parse::<u64>()
        .unwrap();

    if num1 == num2 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_id1() {
        let result = is_valid_id(10);
        assert!(result);
    }

    #[test]
    fn test_is_valid_id2() {
        let result = is_valid_id(44);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_valid_id3() {
        let result = is_valid_id(444);
        assert!(result);
    }

    #[test]
    fn test_is_valid_id4() {
        let result = is_valid_id(1188511885);
        assert_eq!(result, false);
    }
}
