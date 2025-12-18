// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn largest_rectangle(file_path: &String) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let points = load_points(file_contents);

    let mut max_area = 0;

    for (i, (xi, yi)) in points.iter().enumerate() {
        for (xj, yj) in points[i + 1..].iter() {
            let area = (((xj - xi).abs() + 1) as i64) * (((yj - yi).abs() + 1) as i64);
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn load_points(file_contents: String) -> Vec<(i32, i32)> {
    let mut points = vec![];

    for line in file_contents.lines() {
        let mut id_iter = line.split(",");

        let x = id_iter.next().unwrap().parse::<i32>().unwrap();
        let y = id_iter.next().unwrap().parse::<i32>().unwrap();

        points.push((x, y));
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle() {
        let result = largest_rectangle(&String::from("../resources/test-input/day09-test.txt"));
        assert_eq!(result, 50);
    }
}
