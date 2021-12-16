// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn count_depth_inc_slider(file_path: &String) -> i64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let lines = contents.lines();

    let numbers: Vec<i64> = lines.map(|x| x.parse::<i64>().unwrap()).collect();

    let windows = numbers.windows(3);

    let mut previous_total: i64 = i64::MAX;
    let mut count: i64 = 0;

    for window in windows {
        let window_total: i64 = window.iter().sum();

        if window_total > previous_total {
            count += 1;
        }

        previous_total = window_total;
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
