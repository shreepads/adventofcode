// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

fn main() {
    println!(
        "Day 1, #1: {}",
        day1::num_dial_stop_zero(&String::from("resources/input/day01.txt"))
    );

    println!(
        "Day 1, #2: {}",
        day1::num_dial_at_zero(&String::from("resources/input/day01.txt"))
    );
}
