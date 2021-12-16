// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn count_depth_inc(file_path: &String) -> i64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut previous: i64 = i64::MAX;
    let mut count: i64 = 0;

    for line in lines {
        let number: i64 = line.parse::<i64>().unwrap();

        if number > previous {
            count += 1;
        }

        previous = number;
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
