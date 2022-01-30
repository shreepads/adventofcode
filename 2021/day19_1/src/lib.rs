// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod probe;
mod rotations;
mod space;

use std::collections::VecDeque;
use std::fs;

use probe::OtherScanner;
use probe::ZeroScanner;
use rotations::MAX_ROT;
use space::Point;
use space::Translation;

pub fn calculate_beacon_count(file_path: &String) -> (usize, i32) {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut scanner0: ZeroScanner = ZeroScanner::new();

    let mut scanners: VecDeque<OtherScanner> = VecDeque::new();

    load_data(&mut scanner0, &mut scanners, contents);

    let mut merged_scanners: Vec<OtherScanner> = Vec::new();

    while let Some(mut scannern) = scanners.pop_back() {
        let (max_matching_count, max_rot, max_trans) = find_matching_beacons(&scanner0, &scannern);

        if max_matching_count >= 12 {
            merge(&mut scanner0, &mut scannern, max_rot, max_trans);

            merged_scanners.push(scannern);
        } else {
            scanners.push_front(scannern);
        }
    }

    (
        scanner0.beacons.len(),
        largest_manhattan_distance(&merged_scanners),
    )
}

fn largest_manhattan_distance(merged_scanners: &Vec<OtherScanner>) -> i32 {
    let mut max_man_dist = 0i32;

    for (i, scanner1) in merged_scanners.iter().enumerate() {
        for (j, scanner2) in merged_scanners.iter().enumerate() {
            if j >= i {
                // man distance the same both ways
                continue;
            }

            let loc1 = scanner1.location.unwrap();
            let loc2 = scanner2.location.unwrap();
            let man_dist = loc1.manhattan_distance(loc2);

            if man_dist > max_man_dist {
                max_man_dist = man_dist;
            }
        }
    }

    max_man_dist
}

fn merge(
    scanner0: &mut ZeroScanner,
    scannern: &mut OtherScanner,
    max_rot: usize,
    max_trans: Translation,
) {
    for rot_point in scannern.beacon_rotations[max_rot].iter() {
        let trans_rot_point = rot_point.translate_new(max_trans);

        if !scanner0.beacons.contains(&trans_rot_point) {
            // add to scanner0
            scanner0.beacons.insert(trans_rot_point);
        }
    }

    // Set scannern's location (as per scanner0 ref)
    scannern.location = Some(Point::new_zero().translate_new(max_trans));
}

fn find_matching_beacons(
    scanner0: &ZeroScanner,
    scannern: &OtherScanner,
) -> (usize, usize, Translation) {
    let mut max_matching_count = 0;
    let mut max_rot = usize::MAX;
    let mut _max_point0 = Point::new_zero();
    let mut _max_point = Point::new_zero();
    let mut max_trans = Translation::new_zero();

    'outer: for rot in 0..=MAX_ROT {
        for point0 in scanner0.beacons.iter() {
            for (i, rot_point) in scannern.beacon_rotations[rot].iter().enumerate() {
                // check alignments for given pair of points
                let point = scannern.beacons[i];

                // find translation to scanner0 reference
                let trans = rot_point.translate_to(*point0);

                /*
                let trans_points: Vec<Point> = scannern.beacon_rotations[rot]
                    .iter()
                    .map(|x| x.translate_new(trans))
                    .collect();
                */

                let matching_count = scannern.beacon_rotations[rot]
                    .iter()
                    .filter(|x| scanner0.beacons.contains(&x.translate_new(trans)))
                    .count();

                if matching_count > max_matching_count {
                    max_matching_count = matching_count;
                    max_rot = rot;
                    _max_point0 = *point0;
                    _max_point = point;
                    max_trans = trans;

                    if max_matching_count >= 12 {
                        break 'outer;
                    }
                }
            }
        }
    }

    (max_matching_count, max_rot, max_trans)
}

fn load_data(scanner0: &mut ZeroScanner, scanners: &mut VecDeque<OtherScanner>, contents: String) {
    let mut lines = contents.lines();

    // Load scanner 0
    lines.next(); // skip scanner line

    let mut scanner0_loaded = false;
    let mut otherscanner = OtherScanner::new();

    for line in lines {
        if !scanner0_loaded && line.trim() == "" {
            scanner0_loaded = true;
            continue;
        }

        if scanner0_loaded && line.trim() == "" {
            scanners.push_back(otherscanner);
            otherscanner = OtherScanner::new();
            continue;
        }

        if !scanner0_loaded && line.contains("---") {
            continue;
        }

        if scanner0_loaded && line.contains("---") {
            continue;
        }

        if !scanner0_loaded {
            let point = Point::new(line.to_string());
            scanner0.beacons.insert(point);
        }

        if scanner0_loaded {
            let point = Point::new(line.to_string());
            otherscanner.beacons.push(point);
            for (i, rot_point) in point.rotate_vec().iter().enumerate() {
                otherscanner.beacon_rotations[i].push(*rot_point);
            }
        }
    }

    // collect the last one
    scanners.push_back(otherscanner);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day19_1() {
        let result =
            calculate_beacon_count(&String::from("../resources/tests/day19-1-testdata.txt"));
        assert_eq!(result, (79, 3621));
    }
}
