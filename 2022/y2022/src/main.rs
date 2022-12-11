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

    println!(
        "Day 2, #1: {}",
        day2::strategy_guide_score(&String::from("resources/input/day02-1.txt"))
    );

    println!(
        "Day 2, #2: {}",
        day2::correct_strategy_guide_score(&String::from("resources/input/day02-1.txt"))
    );

    println!(
        "Day 3, #1: {}",
        day3::total_priorities_of_common_types(&String::from("resources/input/day03-1.txt"))
    );

    println!(
        "Day 3, #2: {}",
        day3::total_group_badge_priorities(&String::from("resources/input/day03-1.txt"))
    );
}
