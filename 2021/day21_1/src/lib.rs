// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub fn calculate_meaningless_metric(p1_start_pos: u32, p2_start_pos: u32, win_score: u32) -> u32 {

    let mut p1_pos = p1_start_pos;
    let mut p2_pos = p2_start_pos;
    let mut p1_score = 0u32;
    let mut p2_score = 0u32;

    let mut throw = 1u32;
    let mut dice_mid_val = 2u32;

    while p1_score < win_score  &&  p2_score < win_score {
        let throw_val: u32;

        match dice_mid_val {
            1   => {
                throw_val = 103;
                dice_mid_val = 4;
            },
            98 => {
                throw_val = 98 * 3;
                dice_mid_val = 1;
            },
            99 => {
                throw_val = 99 * 3;
                dice_mid_val = 2;
            },
            100 => {
                throw_val = 200;
                dice_mid_val = 3;
            },
            _   => {
                throw_val = dice_mid_val * 3;
                dice_mid_val += 3;
            },
        }

        if throw % 2 == 1
        {
            p1_pos = ((p1_pos + throw_val - 1) % 10) + 1;
            p1_score += p1_pos;
        } else {
            p2_pos = ((p2_pos + throw_val - 1) % 10) + 1;
            p2_score += p2_pos;
        }

        throw += 1;
        
    }

    if p1_score >= win_score {
        return p2_score * (throw-1) * 3;
    } else if p2_score >= win_score {
        return p1_score * (throw-1) * 3;
    } else {
        println!("Something's gone wrong");
    }

    0
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn play48() {
        let result = calculate_meaningless_metric(4, 8, 1000);
        assert_eq!(result, 739785);
    }
}
