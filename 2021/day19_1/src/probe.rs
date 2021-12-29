// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::fmt;

use crate::rotations::MAX_ROT;
use crate::space::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct ZeroScanner {
    pub location: Point,
    pub beacons: HashSet<Point>,
}

impl ZeroScanner {
    pub fn new() -> ZeroScanner {
        ZeroScanner {
            location: Point::new_zero(),
            beacons: HashSet::new(),
        }
    }
}

impl fmt::Display for ZeroScanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ZeroScanner: location {}, {} beacons",
            self.location,
            self.beacons.len()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OtherScanner {
    pub location: Option<Point>,
    pub beacons: Vec<Point>,
    pub beacon_rotations: Vec<Vec<Point>>,
}

impl OtherScanner {
    pub fn new() -> OtherScanner {
        let mut beacon_rotations: Vec<Vec<Point>> = Vec::new();

        for _ in 0..=MAX_ROT {
            beacon_rotations.push(Vec::new());
        }

        OtherScanner {
            location: None,
            beacons: Vec::new(),
            beacon_rotations: beacon_rotations,
        }
    }
}

impl fmt::Display for OtherScanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OtherScanner: location {:?}, {} beacons (first: {}, last: {}), {} rotations",
            self.location,
            self.beacons.len(),
            self.beacons[0],
            self.beacons[self.beacons.len() - 1],
            self.beacon_rotations[0].len()
        )
    }
}
