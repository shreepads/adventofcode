// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod packet;

use packet::Packet;
use std::fs;

pub fn calculate_total_version_no(file_path: &String) -> (u32, u64) {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let packet = Packet::new(contents);

    (packet.version_sum(), packet.value())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
