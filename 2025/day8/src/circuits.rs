// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use crate::{point_pairs::PointPair, points::Point};
use std::{cmp::Ordering, collections::HashSet, fmt};

// Creating as Rust can't handle HashSet<HashSet<Point>>
// Instead use BTreeSet<Circuit> and get ordering by size
// Make the HashSet member public to avoid reimplementing HashSet methods
#[derive(Debug, Clone)]
pub struct Circuit {
    pub points: HashSet<Point>,
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}]", self.points)
    }
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl Eq for Circuit {}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Order Circuits by number of points for use in BTreeSet
impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> Ordering {
        // First by number of elements
        if self.points.len() != other.points.len() {
            return self.points.len().cmp(&other.points.len());
        }

        // If same number of elements check if equal points
        if self.points == other.points {
            return Ordering::Equal;
        }

        // Return one fixed value
        Ordering::Greater
    }
}

// Construct from Point iterator
impl FromIterator<Point> for Circuit {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        let mut points = HashSet::new();

        for i in iter {
            points.insert(i);
        }

        Circuit { points }
    }
}

impl Circuit {
    pub fn new(point_pair: PointPair) -> Circuit {
        let point1 = point_pair.points().0;
        let point2 = point_pair.points().1;

        let mut points = HashSet::new();
        points.insert(point1);
        points.insert(point2);

        Circuit { points }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_ordering_neq() {
        let point1 = Point::_new(1, 5, 1);
        let point2 = Point::_new(2, 2, 3);

        let point_pair1 = PointPair::new(point1, point2);
        let circuit1 = Circuit::new(point_pair1);

        let point3 = Point::_new(3, 3, 3);
        let point4 = Point::_new(0, 5, 2);

        let point_pair2 = PointPair::new(point3, point4);
        let circuit2 = Circuit::new(point_pair2);

        assert_ne!(circuit1, circuit2);
    }

    #[test]
    fn test_circuit_ordering_lt() {
        let point1 = Point::_new(1, 5, 1);
        let point2 = Point::_new(2, 2, 3);

        let point_pair1 = PointPair::new(point1, point2);
        let circuit1 = Circuit::new(point_pair1);

        let point3 = Point::_new(3, 3, 3);
        let point4 = Point::_new(0, 5, 2);
        let point5 = Point::_new(1, 4, 0);

        let point_pair2 = PointPair::new(point3, point4);
        let mut circuit2 = Circuit::new(point_pair2);

        circuit2.points.insert(point5);

        assert!(circuit1 < circuit2);
    }
}
