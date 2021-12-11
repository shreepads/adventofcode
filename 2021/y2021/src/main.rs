use day1_1;
use day1_2;

fn main() {
    println!("Hello, world!");
    println!("Day 1, #1:{}",
        day1_1::count_depth_inc(&String::from("resources/day1-1-input.txt"))
    );
    println!("Day 1, #2:{}",
        day1_2::count_depth_inc_slider(&String::from("resources/day1-1-input.txt"))
    );
}
