// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use crate::points::Point;
use std::{cmp::Ordering, collections::BTreeSet, fmt};

#[derive(Debug, Clone, Copy)]
pub struct PointPair {
    point1: Point,
    point2: Point,
    euc_dist_sq: i64,
}

impl fmt::Display for PointPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}]",
            self.point1, self.point2, self.euc_dist_sq
        )
    }
}

// Not entirely right but works
impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        self.euc_dist_sq == other.euc_dist_sq
    }
}

impl Eq for PointPair {}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Order PointPairs by euclidean distance for finding closest in BTreeSet
impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.euc_dist_sq.cmp(&other.euc_dist_sq)
    }
}

impl PointPair {
    pub fn new(point1: Point, point2: Point) -> PointPair {
        PointPair {
            point1,
            point2,
            euc_dist_sq: point1.euc_dist_sq(point2),
        }
    }

    pub fn points(&self) -> (Point, Point) {
        (self.point1, self.point2)
    }

    pub fn find_n_closest_pairs(points: &Vec<Point>, n: usize) -> BTreeSet<PointPair> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_pair_ordering_eq() {
        let point1 = Point::_new(1, 5, 1);
        let point2 = Point::_new(2, 2, 3);

        let point_pair1 = PointPair::new(point1, point2);

        let point3 = Point::_new(3, 3, 3);
        let point4 = Point::_new(0, 5, 2);

        let point_pair2 = PointPair::new(point3, point4);

        assert_eq!(point_pair1, point_pair2);
    }

    #[test]
    fn test_point_pair_ordering_lt() {
        let point1 = Point::_new(1, 5, 1);
        let point2 = Point::_new(2, 2, 3);

        let point_pair1 = PointPair::new(point1, point2);

        let point3 = Point::_new(3, 2, 3);
        let point4 = Point::_new(0, 5, 2);

        let point_pair2 = PointPair::new(point3, point4);

        assert!(point_pair1 < point_pair2);
    }
}
