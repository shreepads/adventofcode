// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

mod point_pairs;
mod points;

use point_pairs::PointPair;
use points::Point;

pub fn three_largest_circuits_mul(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let points = load_points(file_contents);

    0
}

fn load_points(file_contents: String) -> Vec<Point> {
    let mut points = vec![];

    for point_str in file_contents.lines() {
        points.push(Point::from_str(point_str).unwrap());
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_largest_circuits_mul() {
        let result =
            three_largest_circuits_mul(&String::from("../resources/test-input/day08-test.txt"));
        assert_eq!(result, 40);
    }
}
