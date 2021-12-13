// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    grid_data: [[u32; 1000]; 1000],
}

impl Grid {

    pub fn new() -> Grid {
        Grid {
            grid_data: [[0u32; 1000]; 1000],
        }
    }

}