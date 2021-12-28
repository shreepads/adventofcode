// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod space;

use std::fs;
use space::Cuboid;
use space::Intersection;

pub fn calculate_cubes_on(file_path: &String, max_limit: i32) -> u32 {
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
            } else {
                println!("Cuboid outside limit: {}", line)
            }
        } else if line.contains("off") {
            if let Some(cb) = Cuboid::new(line.to_string(), max_limit) {
                switch_off(&mut plus_cuboids, &mut minus_cuboids, cb);
            } else {
                println!("Cuboid outside limit: {}", line)
            }
        } else {
            println!("Invalid line: {}", line)
        }
    }

    println!("Plus cuboids: {:?}", plus_cuboids);
    println!("Minus cuboids: {:?}", minus_cuboids);

    0
}

fn switch_on(plus_cuboids: &mut Vec<Cuboid>, minus_cuboids: &mut Vec<Cuboid>, new_cb: Cuboid) {

    println!("Switching on {}", new_cb);
    
    let mut new_plus_cuboids: Vec<Cuboid> = Vec::new();
    let mut new_minus_cuboids: Vec<Cuboid> = Vec::new();

    new_plus_cuboids.push(new_cb);

    for pcb in plus_cuboids.iter() {
        match pcb.intersect(&new_cb) {
            Intersection::Null           => {}, // if new cuboid doesn't intersect, nothing to do
            Intersection::Subset(sbcb)   => new_minus_cuboids.push(sbcb), 
            Intersection::Superset(spcb) => new_minus_cuboids.push(spcb),
            Intersection::Overlap(ocb)   => new_minus_cuboids.push(ocb),
        }
    }

    for mcb in minus_cuboids.iter() {
        match mcb.intersect(&new_cb) {
            Intersection::Null           => {}, // if new cuboid doesn't intersect, nothing to do
            Intersection::Subset(sbcb)   => new_plus_cuboids.push(sbcb), 
            Intersection::Superset(spcb) => new_plus_cuboids.push(spcb),
            Intersection::Overlap(ocb)   => new_plus_cuboids.push(ocb),
        }
    }

    plus_cuboids.append(&mut new_plus_cuboids);
    minus_cuboids.append(&mut new_minus_cuboids);

}

fn switch_off(_plus_cuboids: &mut Vec<Cuboid>, _minus_cuboids: &mut Vec<Cuboid>, _new_cb: Cuboid) {
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tiny_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-1-testdata1.txt"), 50);
        assert_eq!(result, 39);
    }

/*
    #[test]
    fn small_test() {
        let result = calculate_cubes_on(&String::from("../resources/tests/day22-1-testdata2.txt"), 50);
        assert_eq!(result, 590784);
    }
    */
}
