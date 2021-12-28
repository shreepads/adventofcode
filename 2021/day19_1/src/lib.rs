// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod space;
mod rotations;
mod probe;

use std::fs;
use std::collections::VecDeque;

use space::Point;
use rotations::MAX_ROT;
use probe::ZeroScanner;
use probe::OtherScanner;

pub fn calculate_beacon_count(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut scanner0: ZeroScanner = ZeroScanner::new();

    let mut scanners: VecDeque<OtherScanner> = VecDeque::new();

    load_data(&mut scanner0, &mut scanners, contents);


    println!("Loaded scanner0: {}", scanner0);

    for (i, scanner) in scanners.iter().enumerate() {
        println!("Loaded other scanner {}: {}", i+1, scanner);
    }

    find_matching_beacons(&scanner0, &scanners[0]);

    0
}

fn find_matching_beacons(scanner0: &ZeroScanner, scannern: &OtherScanner) {

    let mut max_matching_count = 0;
    let mut max_rot = usize::MAX;
    let mut max_point0 = Point::new_zero();
    let mut max_point = Point::new_zero();

    for rot in 0..=MAX_ROT {
        for point0 in scanner0.beacons.iter() {
            for (i, rot_point) in scannern.beacon_rotations[rot].iter().enumerate() {
                // check alignments for given pair of points
                let point = scannern.beacons[i];
                                
                // find translation to scanner0 reference
                let trans = rot_point.translate_to(*point0);

                let trans_points: Vec<Point> = scannern.beacon_rotations[rot].iter()
                    .map(|x| x.translate_new(trans)).collect();

                let matching_count = trans_points.iter()
                    .filter(|x| scanner0.beacons.contains(x)).count();

                println!("Rot {}, point 0 {}, point {}, rot_point {}: {} matching",
                    rot, point0, point, rot_point, matching_count);

                if matching_count > max_matching_count {
                    max_matching_count = matching_count;
                    max_rot = rot;
                    max_point0 = *point0;
                    max_point = point;
                }
            }
        }
    }

    println!("Max match: rot {}, point 0 {}, point {}, {} matching",
        max_rot, max_point0, max_point, max_matching_count);

}

fn load_data(scanner0: &mut ZeroScanner, scanners: &mut VecDeque<OtherScanner>, contents: String) {
    
    let mut lines = contents.lines();

    // Load scanner 0
    lines.next();  // skip scanner line

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

        if !scanner0_loaded  &&  line.contains("---") {
            continue;
        }

        if scanner0_loaded  &&  line.contains("---") {
            
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
        let result = calculate_beacon_count(&String::from("../resources/tests/day19-1-testdata.txt"));
        assert_eq!(result, 79);
    }

}
