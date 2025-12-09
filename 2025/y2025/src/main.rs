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

    println!(
        "Day 2, #1: {}",
        day2::sum_invalid_ids(&String::from("resources/input/day02.txt"))
    );

    println!(
        "Day 2, #2: {}",
        day2::sum_invalid_ids2(&String::from("resources/input/day02.txt"))
    );

    println!(
        "Day 3, #1: {}",
        day3::total_joltage(&String::from("resources/input/day03.txt"))
    );

    println!(
        "Day 3, #2: {}",
        day3::super_total_joltage(&String::from("resources/input/day03.txt"))
    );

    println!(
        "Day 4, #1: {}",
        day4::reachable_rolls(&String::from("resources/input/day04.txt"))
    );

    println!(
        "Day 4, #2: {}",
        day4::ultimately_reachable_rolls(&String::from("resources/input/day04.txt"))
    );

    println!(
        "Day 5, #1: {}",
        day5::fresh_ingredients(&String::from("resources/input/day05.txt"))
    );
}
