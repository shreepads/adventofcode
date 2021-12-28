// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;
use std::collections::HashMap;

pub fn calculate_lit_pixels(file_path: &String, passes: u8) -> u32 {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut parts = contents.split("\n\n");

    let mapping_str = parts.next().unwrap();
    let image_str = parts.next().unwrap();

    let mut mapping : HashMap<usize,char> = HashMap::new();

    load_mapping(&mut mapping, mapping_str.to_string());

    let mut image: [[char; 120]; 120] = [['.'; 120]; 120];

    load_image(&mut image, image_str.to_string());

    for _ in 1..passes {
        enhance(&mut image, &mapping);
    }

    0
}

fn enhance(image: &mut [[char; 120]; 120], mapping: &HashMap<usize,char>) {

}


fn load_mapping(mapping: &mut HashMap<usize,char>, mapping_str: String) {

    for (i, val) in mapping_str.chars().enumerate() {
        mapping.insert(i, val);
    }
}

fn load_image(image: &mut [[char; 120]; 120], image_str: String) {

    for (row, line) in image_str.lines().enumerate() {
        for (col, pixel) in line.chars().enumerate() {
            image[row+10][col+10] = pixel;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
