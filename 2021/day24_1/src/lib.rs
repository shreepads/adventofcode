// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

// Implementation of the logic at https://www.ericburden.work/blog/2022/01/05/advent-of-code-2021-day-24/

pub const INPUTS: usize = 14;
pub const DX: [i64; INPUTS] = [14, 14, 14, 12, 15, -12, -12, 12, -7, 13, -8, -5, -10, -7];
pub const DY: [i64; INPUTS] = [14, 2, 1, 13, 5, 5, 5, 9, 3, 13, 2, 1, 11, 8];
pub const DZ: [i64; INPUTS] = [1, 1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 26, 26, 26];

struct StackElement {
    posn: usize,
    offset: i64,
}

pub fn calculate_max_serialno() -> i64 {
    let mut stack: Vec<StackElement> = Vec::new();

    let mut serial: [i64; INPUTS] = [9; INPUTS];

    for i in 0..INPUTS {
        if DZ[i] == 1 {
            // insert onto stack
            stack.push(StackElement {
                posn: i,
                offset: DY[i],
            });
        } else {
            // pop from stack, assume 26
            if let Some(prev) = stack.pop() {
                let delta = prev.offset + DX[i];

                if delta > 0 {
                    serial[prev.posn] = 9 - delta;
                } else {
                    // assume delta < 0
                    serial[i] = 9 + delta;
                }
            } else {
                println!("Mismatched element {}", i);
            }
        }
    }

    serial.iter().fold(0i64, |acc, x| acc * 10 + x)
}

pub fn calculate_min_serialno() -> i64 {
    let mut stack: Vec<StackElement> = Vec::new();

    let mut serial: [i64; INPUTS] = [1; INPUTS];

    for i in 0..INPUTS {
        if DZ[i] == 1 {
            // insert onto stack
            stack.push(StackElement {
                posn: i,
                offset: DY[i],
            });
        } else {
            // pop from stack, assume 26
            if let Some(prev) = stack.pop() {
                let delta = prev.offset + DX[i];

                if delta > 0 {
                    serial[i] = 1 + delta;
                } else {
                    // assume delta < 0
                    serial[prev.posn] = 1 - delta;
                }
            } else {
                println!("Mismatched element {}", i);
            }
        }
    }

    serial.iter().fold(0i64, |acc, x| acc * 10 + x)
}

#[cfg(test)]
mod tests {}
