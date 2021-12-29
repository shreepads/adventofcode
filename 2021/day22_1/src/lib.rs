// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod space;

use std::fs;
use space::Cuboid;
use space::Intersection;

pub fn calculate_cubes_on(file_path: &String, max_limit: i32) -> u64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut plus_cuboids : Vec<Cuboid> = Vec::new();
    let mut minus_cuboids : Vec<Cuboid> = Vec::new();

    for line in contents.lines() {
        if line.contains("on") {
            if let Some(cb) = Cuboid::new(line.to_string(), max_limit) {
                switch_on(&mut plus_cuboids, &mut minus_cuboids, cb);
            }
        } else if line.contains("off") {
            if let Some(cb) = Cuboid::new(line.to_string(), max_limit) {
                switch_off(&mut plus_cuboids, &mut minus_cuboids, cb);
            }
        } else {
            println!("Invalid line: {}", line)
        }
    }

    let mut volume: u64 = plus_cuboids.iter().map(|x| x.volume()).sum();

    volume -= minus_cuboids.iter().map(|x| x.volume()).sum::<u64>();

    volume
}

fn switch_on(plus_cuboids: &mut Vec<Cuboid>, minus_cuboids: &mut Vec<Cuboid>, new_cb: Cuboid) {

    let mut new_plus_cuboids: Vec<Cuboid> = Vec::new();
    let mut new_minus_cuboids: Vec<Cuboid> = Vec::new();

    new_plus_cuboids.push(new_cb);

    for pcb in plus_cuboids.iter() {
        match pcb.intersect(new_cb) {
            Intersection::Null           => {}, // if new cuboid doesn't intersect, nothing to do
            Intersection::Overlap(ocb)   => new_minus_cuboids.push(ocb),
        }
    }

    for mcb in minus_cuboids.iter() {
        match mcb.intersect(new_cb) {
            Intersection::Null           => {}, // if new cuboid doesn't intersect, nothing to do
            Intersection::Overlap(ocb)   => new_plus_cuboids.push(ocb),
        }
    }

    plus_cuboids.append(&mut new_plus_cuboids);
    minus_cuboids.append(&mut new_minus_cuboids);

}

fn switch_off(plus_cuboids: &mut Vec<Cuboid>, minus_cuboids: &mut Vec<Cuboid>, new_cb: Cuboid) {
   
    let mut new_plus_cuboids: Vec<Cuboid> = Vec::new();
    let mut new_minus_cuboids: Vec<Cuboid> = Vec::new();

    for pcb in plus_cuboids.iter() {
        match pcb.intersect(new_cb) {
            Intersection::Null           => {}, // if new cuboid doesn't intersect, nothing to do
            Intersection::Overlap(ocb)   => new_minus_cuboids.push(ocb),
        }
    }

    for mcb in minus_cuboids.iter() {
        match mcb.intersect(new_cb) {
            Intersection::Null           => {}, // if new cuboid doesn't intersect, nothing to do
            Intersection::Overlap(ocb)   => new_plus_cuboids.push(ocb),
        }
    }

    plus_cuboids.append(&mut new_plus_cuboids);
    minus_cuboids.append(&mut new_minus_cuboids);

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tiny_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-1-testdata1.txt"), 50);
        assert_eq!(result, 39);
    }


    #[test]
    fn small_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-1-testdata2.txt"), 50);
        assert_eq!(result, 590784);
    }
    

    #[test]
    fn big_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-2-testdata.txt"), 150000);
        assert_eq!(result, 2758514936282235);
    }
}
