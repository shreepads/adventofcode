// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod space;
mod rotations;
mod probe;

use std::fs;
use std::collections::VecDeque;

use space::Point;
use space::Translation;
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

    let (max_matching_count, max_rot, max_trans) = find_matching_beacons(&scanner0, &scanners[0]);

    if max_matching_count >= 12 {
        println!("Found match with {} points: rot {}, trans {}. MERGING",
            max_matching_count, max_rot, max_trans);
        
        merge(&mut scanner0, &mut scanners[0], max_rot, max_trans);

        println!("Post merge scannern location: {:?}", scanners[0].location);
        println!("Post merge scanner0 : {}", scanner0);

    } else {
        println!("Only found {} points matching", max_matching_count);
    }

    let (max_matching_count_4, max_rot_4, max_trans_4) = find_matching_beacons(&scanner0, &scanners[3]);

    merge(&mut scanner0, &mut scanners[3], max_rot_4, max_trans_4);

    println!("Post merge scannern location: {:?}", scanners[3].location);
    println!("Post merge scanner0 : {}", scanner0);

    0
}

fn merge(scanner0: &mut ZeroScanner, scannern: &mut OtherScanner, max_rot: usize, max_trans: Translation) {

    for (i, rot_point) in scannern.beacon_rotations[max_rot].iter().enumerate() {

        let trans_rot_point = rot_point.translate_new(max_trans);

        if scanner0.beacons.contains(&trans_rot_point) {
            // just print out
            println!("Point {} (rot point {}) already in scanner0 as {}",
                scannern.beacons[i], rot_point, trans_rot_point);
        } else {
            // add to scanner0
            scanner0.beacons.insert(trans_rot_point);
        }
    } 

    // Set scannern's location (as per scanner0 ref)
    scannern.location = Some(Point::new_zero().translate_new(max_trans));
}


fn find_matching_beacons(scanner0: &ZeroScanner, scannern: &OtherScanner) -> (usize, usize, Translation) {

    let mut max_matching_count = 0;
    let mut max_rot = usize::MAX;
    let mut max_point0 = Point::new_zero();
    let mut max_point = Point::new_zero();
    let mut max_trans = Translation::new_zero();

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

                if matching_count > max_matching_count {
                    max_matching_count = matching_count;
                    max_rot = rot;
                    max_point0 = *point0;
                    max_point = point;
                    max_trans = trans;

                    println!("Rot {}, point 0 {}, point {}, rot_point {}, trans {}: {} matching",
                        rot, point0, point, rot_point, trans, matching_count);
                }
            }
        }
    }

    println!("Max match: rot {}, point 0 {}, point {}, trans {}: {} matching",
        max_rot, max_point0, max_point, max_trans, max_matching_count);

    (max_matching_count, max_rot, max_trans)
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
