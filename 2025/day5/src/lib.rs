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

    let mut ranges = load_ranges(splitter.next().unwrap());

    let mut collapsed_ranges: Vec<(u64, u64)> = vec![];

    // Sort ranges so ordered by start point
    ranges.sort();

    // Collapse the sorted ranges

    let mut coll_range_start = 0;
    let mut coll_range_end = 0;

    for (i, range) in ranges.iter().enumerate() {
        // If first range then set collapsed range
        if i == 0 {
            coll_range_start = range.0;
            coll_range_end = range.1;
            continue;
        }

        // Does this range lay one beyond the collapsed range
        if coll_range_end < range.0 - 1 {
            // Insert collapsed range and reset it to this one
            collapsed_ranges.push((coll_range_start, coll_range_end));
            coll_range_start = range.0;
            coll_range_end = range.1;
            continue;
        }

        // Does this range lay within the collapsed range
        if coll_range_end >= range.1 {
            // Nothing to do on this range
            continue;
        }

        // Ranges partly overlap, reset collapsed range end
        coll_range_end = range.1;

        // If last range then insert collapsed range
        if i == ranges.len() - 1 {
            collapsed_ranges.push((coll_range_start, coll_range_end));
        }
    }

    collapsed_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
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
