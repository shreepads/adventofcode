// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_least_fuel(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut crab_posns: Vec<u32> = Vec::new();

    for crab_posn in contents.trim_end().split(",") {
        crab_posns.push(crab_posn.parse::<u32>().unwrap());
    }

    let mut min_fuel = u32::MAX;

    let min_posn = *crab_posns.iter().min().unwrap();  // Need to deref the unwrap
    let max_posn = *crab_posns.iter().max().unwrap();

    for posn in min_posn..=max_posn {
        let fuel = calculate_fuel(&crab_posns, posn);
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

fn calculate_fuel(crab_posns: &Vec<u32>, input_posn: u32) -> u32 {

    let mut fuel = 0u32;

    for posn in crab_posns.iter() {
        if posn < &input_posn {
            fuel += ((input_posn - posn) * (input_posn - posn + 1))/2
        }
        else {
            fuel += ((posn - input_posn) * (posn - input_posn + 1))/2
        }
    }

    fuel

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
