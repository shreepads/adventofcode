// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT


#[derive(Debug, Copy, Clone)]
pub struct Grid {
    grid_data: [[u8; 1000]; 1000],  // u32 causes a stack overflow
}

impl Grid {

    pub fn new() -> Grid {
        Grid {
            grid_data: [[0u8; 1000]; 1000],
        }
    }

    pub fn add_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, add_diagonals: bool) {

        // Exclude diagonals if flagged
        if x1 != x2  &&  y1 != y2 && !add_diagonals {
            return
        }

        // Add vertical line
        if x1 == x2 {
            let mut start = y1;
            let mut end = y2;
            if y1 > y2 {
                start = y2;
                end = y1;
            }
            for y in start..=end {
                self.grid_data[y][x1] += 1;
            }
            return;
        }

        // Add horizontal line
        if y1 == y2 {
            let mut start = x1;
            let mut end = x2;
            if x1 > x2 {
                start = x2;
                end = x1;
            }
            for x in start..=end {
                self.grid_data[y1][x] += 1;
            }
            return;
        }

        if x1 != x2  &&  y1 != y2 && add_diagonals {
            println!("Need to add a diagonal!");
            return;
        }
    }

    pub fn overlap_points(self) -> u32 {
        
        let mut overlap_points = 0u32;

        for row in self.grid_data.iter() {
            overlap_points += row.iter()
                .filter(|x| **x > 1)
                .fold(0, |acc, _| acc + 1);   // using count() causes type mismatch
        }

        overlap_points
    }

}