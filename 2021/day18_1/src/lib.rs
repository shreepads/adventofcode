// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod sfish_math;

use std::fs;
use sfish_math::Number;

pub fn calculate_magnitude_sum(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let sfish_no = Number::new("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".to_string());
    println!("Snailfish #: {:?}", sfish_no);
    
    sfish_no.magnitude()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
