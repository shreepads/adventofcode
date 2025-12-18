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

    println!(
        "Day 5, #2: {}",
        day5::all_fresh_ingredients(&String::from("resources/input/day05.txt"))
    );

    println!(
        "Day 6, #1: {}",
        day6::answer_total(&String::from("resources/input/day06.txt"))
    );

    println!(
        "Day 6, #2: {}",
        day6::correct_answer_total(&String::from("resources/input/day06.txt"))
    );

    println!(
        "Day 7, #1: {}",
        day7::beam_splits(&String::from("resources/input/day07.txt"))
    );

    println!(
        "Day 7, #2: {}",
        day7::timelines(&String::from("resources/input/day07.txt"))
    );

    println!(
        "Day 8, #1: {}",
        day8::three_largest_circuits_mul(&String::from("resources/input/day08.txt"), 1000)
    );

    println!(
        "Day 8, #2: {}",
        day8::last_pair_x_mul(&String::from("resources/input/day08.txt"))
    );

    println!(
        "Day 8, #2: {}",
        day9::largest_rectangle(&String::from("resources/input/day09.txt"))
    );
}
