// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

fn main() {
    println!(
        "Day 1, #1: {}",
        day1::most_cals_carried(&String::from("resources/input/day01-1.txt"))
    );

    println!(
        "Day 1, #2: {}",
        day1::top3_total_cals_carried(&String::from("resources/input/day01-1.txt"))
    );

}
