// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::fs;

pub fn calculate_lit_pixels(file_path: &String, passes: u8) -> usize {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut parts = contents.split("\n\n");

    let mapping_str = parts.next().unwrap();
    let image_str = parts.next().unwrap();

    let mut mapping: HashMap<usize, char> = HashMap::new();

    load_mapping(&mut mapping, mapping_str.to_string());

    let mut image: [[char; 220]; 220] = [['.'; 220]; 220];

    load_image(&mut image, image_str.to_string());

    for i in 1..=passes {
        enhance(&mut image, &mapping, i);
    }

    lit_pixels(&image)
}

fn _print_image(image: &[[char; 220]; 220]) {
    for row in 30..71 {
        for col in 30..71 {
            print!("{}", image[row][col]);
        }
        println!("");
    }
}

fn lit_pixels(image: &[[char; 220]; 220]) -> usize {
    let mut lit_pixels = 0;

    for row in image.iter() {
        lit_pixels += row.iter().filter(|x| **x == '#').count();
    }

    lit_pixels
}

fn enhance(image: &mut [[char; 220]; 220], mapping: &HashMap<usize, char>, pass: u8) {
    let mut fill_char = '.';
    if pass % 2 == 1 {
        fill_char = '#';
    }

    let mut enhanced_image = [[fill_char; 220]; 220];

    for row in 1..219 {
        for col in 1..219 {
            let mut num_str = String::new();
            num_str.push_str(&image[row - 1][col - 1..=col + 1].iter().collect::<String>());
            num_str.push_str(&image[row][col - 1..=col + 1].iter().collect::<String>());
            num_str.push_str(&image[row + 1][col - 1..=col + 1].iter().collect::<String>());

            let mut num = 0usize;
            for (i, pixel) in num_str.chars().enumerate() {
                if pixel == '#' {
                    num += 2usize.pow(8 - i as u32);
                }
            }

            enhanced_image[row][col] = *mapping.get(&num).unwrap();
        }
    }

    *image = enhanced_image;
}

fn load_mapping(mapping: &mut HashMap<usize, char>, mapping_str: String) {
    for (i, val) in mapping_str.chars().enumerate() {
        mapping.insert(i, val);
    }
}

fn load_image(image: &mut [[char; 220]; 220], image_str: String) {
    for (row, line) in image_str.lines().enumerate() {
        for (col, pixel) in line.chars().enumerate() {
            image[row + 60][col + 60] = pixel;
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