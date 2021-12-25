// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod sfish_math;

use std::fs;
use sfish_math::Number;

pub fn calculate_magnitude_sum(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut lines = contents.lines();

    let line = lines.next().unwrap();
    let mut sfish_no = Number::new(line.to_string());

    for nextline in lines {
        let next_fish_no = Number::new(nextline.to_string());
        sfish_no.add(&next_fish_no);
        sfish_no.reduce();
        println!("Added, reduced: {}", sfish_no);
    }
    
    sfish_no.magnitude()
}


#[cfg(test)]
mod tests {

    
    use super::*;
    #[test]
    fn day18_1() {
        let result = calculate_magnitude_sum(&String::from("../resources/tests/day18-1-testdata.txt"));
        assert_eq!(result, 4140);
    }
    

}
