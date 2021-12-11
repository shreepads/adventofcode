use day1_1;
use day1_2;
use day2_1;

fn main() {
    println!("Hello, world!");

    // Day 1, #1
    println!("Day 1, #1:{}",
        day1_1::count_depth_inc(&String::from("resources/day1-1-input.txt"))
    );

    // Day 1, #2
    println!("Day 1, #2:{}",
        day1_2::count_depth_inc_slider(&String::from("resources/day1-1-input.txt"))
    );

    // Day 2, #1
    let start_posn = day2_1::Position {
        horizontal: 0,
        depth: 0,
    };
    let end_posn: day2_1::Position = day2_1::calculate_position(
        start_posn,
        &String::from("resources/day2-1-input.txt")
    );
    println!("Day 2, #1:{}", end_posn.horizontal * end_posn.depth);

    // Day 2, #2
    let start_posn_2 = day2_2::Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    let end_posn_2: day2_2::Position = day2_2::calculate_position(
        start_posn_2,
        &String::from("resources/day2-1-input.txt")
    );
    println!("Day 2, #2:{}", end_posn_2.horizontal * end_posn_2.depth);

}
