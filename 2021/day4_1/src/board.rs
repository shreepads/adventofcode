// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug)]
pub struct Cell {
    number: u32,
    marked: bool,
}

#[derive(Debug)]
pub struct Board {
    data: [[u32; 5]; 5],
}