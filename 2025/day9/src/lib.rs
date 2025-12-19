// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

mod shapes;

use shapes::{Point, Segment, ThickInnerRectangle};

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

pub fn largest_contained_rectangle(file_path: &String) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let points = load_points(file_contents);

    let perimeter_segs = perimeter(&points);

    let mut max_area = 0;

    for (i, (xi, yi)) in points.iter().enumerate() {
        for (xj, yj) in points[i + 1..].iter() {
            // If a thick inner rectangle is formed check if it
            // intersects the perimeter segments
            // Assumes thin rectangle can't be biggest
            if let Some(inner_rect) = ThickInnerRectangle::new(*xi, *yi, *xj, *yj) {
                if !rectangle_intersects_segs(&inner_rect, &perimeter_segs) {
                    let area = (((xj - xi).abs() + 1) as i64) * (((yj - yi).abs() + 1) as i64);
                    if area > max_area {
                        max_area = area;
                        println!("({},{})-({},{})", xi, yi, xj, yj);
                    }
                }
            }
        }
    }
    max_area
}

fn perimeter(points: &Vec<(i32, i32)>) -> Vec<Segment> {
    let mut perimeter_segs = vec![];

    for segment_points in points.windows(2) {
        let seg = Segment::new(
            segment_points[0].0,
            segment_points[0].1,
            segment_points[1].0,
            segment_points[1].1,
        );

        perimeter_segs.push(seg);
    }

    // Add start and end
    let close_seg = Segment::new(
        points[0].0,
        points[0].1,
        points[points.len() - 1].0,
        points[points.len() - 1].1,
    );

    perimeter_segs.push(close_seg);

    perimeter_segs
}

fn rectangle_intersects_segs(rect: &ThickInnerRectangle, segments: &Vec<Segment>) -> bool {
    for segment in segments {
        if rect.intersects(segment) {
            return true;
        }
    }

    false
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

    #[test]
    fn test_largest_contained_rectangle() {
        let result =
            largest_contained_rectangle(&String::from("../resources/test-input/day09-test.txt"));
        assert_eq!(result, 24);
    }
}
