// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn calculate_gamma_epsilon(file_path: &String) -> (i64, i64) {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines();

    // let _length = lines.by_ref().count() doesn't work
    // Add another item to count length

    let first_line = match lines.next() {
        Some(line) => line,
        None       => {
            println!("No first line found");
            ""
        }
    };

    let length = first_line.len() + 1;

    // Init vec with 0's
    let mut counts: Vec<i64> = vec![0; length];

    add_line(&mut counts, first_line, length);

    for line in lines {
        add_line(&mut counts, line, length);
    }

    println!("Counts:{:?}", counts);

    gamma_epsilon(counts, length)

}

fn add_line(counts: &mut Vec<i64>, line: &str, size: usize) {

    for (posn, charac) in line.char_indices() {
        match charac.to_digit(2) {
            Some(1) => counts[posn] += 1,
            None    => println!("Non binary digit in {}", line),
            _       => {},
        }
    };

    counts[size - 1] += 1;
}

fn gamma_epsilon(counts: Vec<i64>, length: usize) -> (i64, i64) {

    let mut gamma: i64 = 0;
    let mut epsilon: i64 = 0;

    let total_rows = counts[length - 1];

    for i in 0..length-1 {
        let power: u32 = (length - 2- i).try_into().unwrap();

        if counts[i] > total_rows/2 {
            gamma += 2i64.pow(power);
        }
        else {
            epsilon += 2i64.pow(power);
        }
    }

    (gamma, epsilon)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
