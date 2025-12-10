// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn fresh_ingredients(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut splitter = file_contents.split("\n\n");

    let ranges = load_ranges(splitter.next().unwrap());

    let mut fresh_ingredients_count = 0;

    for id_str in splitter.next().unwrap().lines() {
        let id = id_str.parse::<u64>().unwrap();

        for range in ranges.iter() {
            if id >= range.0 && id <= range.1 {
                fresh_ingredients_count += 1;
                break;
            }
        }
    }

    fresh_ingredients_count
}

pub fn all_fresh_ingredients(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut splitter = file_contents.split("\n\n");

    let ranges = load_ranges(splitter.next().unwrap());

    // TODO: Collapse the ranges

    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn load_ranges(ranges_str: &str) -> Vec<(u64, u64)> {
    let mut ranges = vec![];

    for range_str in ranges_str.lines() {
        let mut id_iter = range_str.split("-");

        let start_id = id_iter.next().unwrap().parse::<u64>().unwrap();
        let end_id = id_iter.next().unwrap().parse::<u64>().unwrap();

        ranges.push((start_id, end_id));
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_fresh_ingredients() {
        let result = all_fresh_ingredients(&String::from("../resources/test-input/day05-test.txt"));
        assert_eq!(result, 14);
    }

    #[test]
    fn test_load_ranges() {
        let result = load_ranges(
            r#"3-5
10-14
16-20
12-18"#,
        );
        assert_eq!(result, vec!((3, 5), (10, 14), (16, 20), (12, 18)));
    }
}
