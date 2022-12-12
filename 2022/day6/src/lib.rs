// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use ahash::AHashSet;
use std::fs;

pub fn start_of_packet_marker(file_path: &String, window_size: usize) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    // Obviously a windows() function on a String/iterator would be too easy

    let mut signal: Vec<char> = Vec::new();

    file_contents.chars().for_each(|ch| signal.push(ch));

    for (i, char_window) in signal.windows(window_size).enumerate() {
        let mut char_set = AHashSet::new();

        char_window.iter().for_each(|ch| {
            char_set.insert(ch);
        });

        if char_set.len() == window_size {
            return i + window_size;
        }
    }

    0
}
