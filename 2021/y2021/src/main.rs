// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use day10_1;
use day10_2;
use day11_1;
use day1_1;
use day1_2;
use day2_1;
use day3_1;
use day3_2;
use day4_1;
use day4_2;
use day5_1;
use day5_2;
use day6_1;
use day6_2;
use day7_1;
use day7_2;
use day8_1;
use day8_2;
use day9_1;
use day9_2;

fn main() {
    // Day 1, #1
    println!(
        "Day 1, #1: {}",
        day1_1::count_depth_inc(&String::from("resources/day1-1-input.txt"))
    );

    // Day 1, #2
    println!(
        "Day 1, #2: {}",
        day1_2::count_depth_inc_slider(&String::from("resources/day1-1-input.txt"))
    );

    // Day 2, #1
    let start_posn = day2_1::Position {
        horizontal: 0,
        depth: 0,
    };
    let end_posn: day2_1::Position =
        day2_1::calculate_position(start_posn, &String::from("resources/day2-1-input.txt"));
    println!("Day 2, #1: {}", end_posn.horizontal * end_posn.depth);

    // Day 2, #2
    let start_posn_2 = day2_2::Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    let end_posn_2: day2_2::Position =
        day2_2::calculate_position(start_posn_2, &String::from("resources/day2-1-input.txt"));
    println!("Day 2, #2: {}", end_posn_2.horizontal * end_posn_2.depth);

    // Day 3, #1
    let (gamma, epsilon) =
        day3_1::calculate_gamma_epsilon(&String::from("resources/day3-1-input.txt"));
    println!("Day 3, #1: {}", gamma * epsilon);

    // Day 3, #2
    let (oxygen, co2) = day3_2::calculate_oxygen_co2(&String::from("resources/day3-1-input.txt"));
    println!("Day 3, #2: {}", oxygen * co2);

    // Day 4, #1
    println!(
        "Day 4, #1: {}",
        day4_1::calculate_winning_board_score(&String::from("resources/day4-1-input.txt"))
    );

    // Day 4, #2
    println!(
        "Day 4, #2: {}",
        day4_2::calculate_last_winning_board_score(&String::from("resources/day4-1-input.txt"))
    );

    // Day 5, #1
    println!(
        "Day 5, #1: {}",
        day5_1::calculate_overlap_points(&String::from("resources/day5-1-input.txt"))
    );

    // Day 5, #2
    println!(
        "Day 5, #2: {}",
        day5_2::calculate_overlap_points(&String::from("resources/day5-1-input.txt"))
    );

    // Day 6, #1
    println!(
        "Day 6, #1: {}",
        day6_1::calculate_fish_population(&String::from("resources/day6-1-input.txt"), 80)
    );

    // Day 6, #2
    println!(
        "Day 6, #2: {}  NOT SOLVED",
        day6_2::calculate_fish_population(&String::from("resources/day6-1-input.txt"), 256)
    );

    // Day 7, #1
    println!(
        "Day 7, #1: {}",
        day7_1::calculate_least_fuel(&String::from("resources/day7-1-input.txt"))
    );

    // Day 7, #2
    println!(
        "Day 7, #2: {}",
        day7_2::calculate_least_fuel(&String::from("resources/day7-1-input.txt"))
    );

    // Day 8, #1
    println!(
        "Day 8, #1: {}",
        day8_1::count_output_digits(&String::from("resources/day8-1-input.txt"))
    );

    // Day 8, #2
    println!(
        "Day 8, #2: {}",
        day8_2::calculate_outputs_total(&String::from("resources/day8-1-input.txt"))
    );

    // Day 9, #1
    println!(
        "Day 9, #1: {}",
        day9_1::calculate_risk_lowpoints(&String::from("resources/day9-1-input.txt"))
    );

    // Day 9, #2
    println!(
        "Day 9, #2: {}",
        day9_2::top3_basins_product(&String::from("resources/day9-1-input.txt"))
    );

    // Day 10, #1
    println!(
        "Day 10, #1: {}",
        day10_1::calculate_total_syntaxerror_score(&String::from("resources/day10-1-input.txt"))
    );

    // Day 10, #2
    println!(
        "Day 10, #2: {}",
        day10_2::median_completion_score(&String::from("resources/day10-1-input.txt"))
    );

    // Day 11, #1
    println!(
        "Day 11, #1: {}",
        day11_1::calculate_total_flashes(&String::from("resources/day11-1-input.txt"), 10)
    );

}
