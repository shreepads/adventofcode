// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::{collections::BTreeSet, fs};

mod point_pairs;
mod points;

use point_pairs::PointPair;
use points::Point;

pub fn three_largest_circuits_mul(file_path: &String, n: usize) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let points = load_points(file_contents);

    let n_closest_pairs = find_n_closest_pairs(points, n);

    for pair in n_closest_pairs {
        println!("{}", pair);
    }

    0
}

fn find_n_closest_pairs(points: Vec<Point>, n: usize) -> BTreeSet<PointPair> {
    let mut n_closest_pairs = BTreeSet::new();

    for (i, point1) in points.iter().enumerate() {
        for point2 in points[i + 1..].iter() {
            let point_pair = PointPair::new(*point1, *point2);

            // If less than n just insert else replace if smaller than largest
            if n_closest_pairs.len() < n {
                n_closest_pairs.insert(point_pair);
            } else {
                if point_pair < *n_closest_pairs.last().unwrap() {
                    n_closest_pairs.pop_last();
                    n_closest_pairs.insert(point_pair);
                }
            }
        }
    }

    n_closest_pairs
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
            three_largest_circuits_mul(&String::from("../resources/test-input/day08-test.txt"), 10);
        assert_eq!(result, 40);
    }
}
