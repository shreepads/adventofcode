// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::{collections::BTreeSet, fs};

mod circuits;
mod point_pairs;
mod points;

use circuits::Circuit;
use point_pairs::PointPair;
use points::Point;

const MAX_CLOSE_PAIRS: usize = 10000;

pub fn three_largest_circuits_mul(file_path: &String, n: usize) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let points = load_points(file_contents);

    let n_closest_pairs = PointPair::find_n_closest_pairs(&points, n);

    let circuits = find_circuits(&n_closest_pairs);

    let mut three_mul = 1;

    for (i, circuit) in circuits.iter().rev().enumerate() {
        if i < 3 {
            three_mul *= circuit.points.len();
        }

        if i == 2 {
            break;
        }
    }

    three_mul
}

pub fn last_pair_x_mul(file_path: &String) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let points = load_points(file_contents);

    let n_closest_pairs = PointPair::find_n_closest_pairs(&points, MAX_CLOSE_PAIRS);

    let last_pair = find_last_pair(&points, &n_closest_pairs).unwrap();

    (last_pair.points().0.x as i64) * (last_pair.points().1.x as i64)
}

fn find_last_pair(
    points: &Vec<Point>,
    closest_point_pairs: &BTreeSet<PointPair>,
) -> Option<PointPair> {
    let mut circuits: BTreeSet<Circuit> = BTreeSet::new();

    for (i, pair) in closest_point_pairs.iter().enumerate() {
        let point1 = pair.points().0;
        let point2 = pair.points().1;

        // First pair of points inserted as a circuit by themselves
        if i == 0 {
            let new_circuit = Circuit::new(*pair);
            circuits.insert(new_circuit);

            let circuit_len = circuits
                .iter()
                .map(|c| c.points.len())
                .fold(0, |acc, x| acc + x);
            if circuit_len == points.len() {
                return Some(*pair);
            }
            continue;
        }

        // Extract circuits containing the points, note point1_circuits could contain point2
        let point1_circuits: Vec<Circuit> = circuits
            .extract_if(.., |x| x.points.contains(&point1))
            .collect();
        let point2_circuits: Vec<Circuit> = circuits
            .extract_if(.., |x| x.points.contains(&point2))
            .collect();

        // Check the number of circuits is <2
        assert!(point1_circuits.len() < 2);
        assert!(point2_circuits.len() < 2);

        // If both points aren't in any circuit add them as a new circuit
        if point1_circuits.len() == 0 && point2_circuits.len() == 0 {
            let new_circuit = Circuit::new(*pair);
            circuits.insert(new_circuit);

            let circuit_len = circuits
                .iter()
                .map(|c| c.points.len())
                .fold(0, |acc, x| acc + x);
            if circuit_len == points.len() {
                return Some(*pair);
            }
            continue;
        }

        // If both points are in a circuit each then merge
        // These have to be different as we are extracting based on contains
        if point1_circuits.len() == 1 && point2_circuits.len() == 1 {
            let point1_circuit = &point1_circuits[0];
            let point2_circuit = &point2_circuits[0];
            let new_circuit = point1_circuit
                .points
                .union(&point2_circuit.points)
                .map(|x| *x)
                .collect();
            circuits.insert(new_circuit);

            let circuit_len = circuits
                .iter()
                .map(|c| c.points.len())
                .fold(0, |acc, x| acc + x);
            if circuit_len == points.len() {
                return Some(*pair);
            }
            continue;
        }

        // If point1 is in a circuit add point2 to it (it might already contain point2)
        if point1_circuits.len() == 1 {
            let mut new_circuit = point1_circuits[0].clone();
            new_circuit.points.insert(point2);
            circuits.insert(new_circuit);

            let circuit_len = circuits
                .iter()
                .map(|c| c.points.len())
                .fold(0, |acc, x| acc + x);
            if circuit_len == points.len() {
                return Some(*pair);
            }
            continue;
        }

        // If point2 is in a circuit add point1 to it (it might already contain point1)
        if point2_circuits.len() == 1 {
            let mut new_circuit = point2_circuits[0].clone();
            new_circuit.points.insert(point1);
            circuits.insert(new_circuit);

            let circuit_len = circuits
                .iter()
                .map(|c| c.points.len())
                .fold(0, |acc, x| acc + x);
            if circuit_len == points.len() {
                return Some(*pair);
            }
            continue;
        }
    }

    None
}

fn find_circuits(closest_point_pairs: &BTreeSet<PointPair>) -> BTreeSet<Circuit> {
    let mut circuits: BTreeSet<Circuit> = BTreeSet::new();

    for (i, pair) in closest_point_pairs.iter().enumerate() {
        let point1 = pair.points().0;
        let point2 = pair.points().1;

        // First pair of points inserted as a circuit by themselves
        if i == 0 {
            let new_circuit = Circuit::new(*pair);
            circuits.insert(new_circuit);
            continue;
        }

        // Extract circuits containing the points, note point1_circuits could contain point2
        let point1_circuits: Vec<Circuit> = circuits
            .extract_if(.., |x| x.points.contains(&point1))
            .collect();
        let point2_circuits: Vec<Circuit> = circuits
            .extract_if(.., |x| x.points.contains(&point2))
            .collect();

        // Check the number of circuits is <2
        assert!(point1_circuits.len() < 2);
        assert!(point2_circuits.len() < 2);

        // If both points aren't in any circuit add them as a new circuit
        if point1_circuits.len() == 0 && point2_circuits.len() == 0 {
            let new_circuit = Circuit::new(*pair);
            circuits.insert(new_circuit);
            continue;
        }

        // If both points are in a circuit each then merge
        // These have to be different as we are extracting based on contains
        if point1_circuits.len() == 1 && point2_circuits.len() == 1 {
            let point1_circuit = &point1_circuits[0];
            let point2_circuit = &point2_circuits[0];
            let new_circuit = point1_circuit
                .points
                .union(&point2_circuit.points)
                .map(|x| *x)
                .collect();
            circuits.insert(new_circuit);
            continue;
        }

        // If point1 is in a circuit add point2 to it (it might already contain point2)
        if point1_circuits.len() == 1 {
            let mut new_circuit = point1_circuits[0].clone();
            new_circuit.points.insert(point2);
            circuits.insert(new_circuit);
            continue;
        }

        // If point2 is in a circuit add point1 to it (it might already contain point1)
        if point2_circuits.len() == 1 {
            let mut new_circuit = point2_circuits[0].clone();
            new_circuit.points.insert(point1);
            circuits.insert(new_circuit);
            continue;
        }
    }

    circuits
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
