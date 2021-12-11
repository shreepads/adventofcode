use day1_1;

fn main() {
    println!("Hello, world!");
    println!("Day 1, #1:{}",
        day1_1::count_depth_inc(&String::from("resources/day1-1-input.txt"))
    );
}
