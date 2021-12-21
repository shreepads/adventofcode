// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

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
        day11_1::calculate_total_flashes(&String::from("resources/day11-1-input.txt"), 100)
    );

    // Day 11, #2
    println!(
        "Day 11, #2: {}",
        day11_2::first_all_flash(&String::from("resources/day11-1-input.txt"), 1000)
    );

    // Day 12, #1
    println!(
        "Day 12, #1: {}",
        day12_1::calculate_total_paths(&String::from("resources/day12-1-input.txt"))
    );

    /* Removed for performance, FIX
    // Day 12, #2
    println!(
        "Day 12, #2: {}",
        day12_2::calculate_total_paths(&String::from("resources/day12-1-input.txt"))
    );
    */

    // Day 13, #1
    println!(
        "Day 13, #1: {}",
        day13_1::calculate_visible_dots(&String::from("resources/day13-1-input.txt"), 1, false)
    );

    // Day 13, #2
    println!(
        "Day 13, #2: {}",
        day13_1::calculate_visible_dots(&String::from("resources/day13-1-input.txt"), 12, true)
    );

    // Day 14, #1
    println!(
        "Day 14, #1: {}",
        day14_1::calculate_element_diff(&String::from("resources/day14-1-input.txt"), 10)
    );

    // Day 15, #1
    println!(
        "Day 15, #1: {}",
        day15_1::calculate_least_risk_path(&String::from("resources/day15-1-input.txt"))
    );

    // Day 15, #2
    println!(
        "Day 15, #2: {}",
        day15_2::calculate_least_risk_path(&String::from("resources/day15-1-input.txt"))
    );

    // Day 16, #1
    println!(
        "Day 16, #1, 2: {:?}",
        day16_1::calculate_total_version_no(&String::from("resources/day16-1-input.txt"))
    );

    // Day 17, #1
    println!(
        "Day 17, #1, 2: {:?}",
        day17_1::calculate_maxy_trajectory(235, 259, -118, -62)
    );
}
