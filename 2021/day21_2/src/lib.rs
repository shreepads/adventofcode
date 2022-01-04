// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub fn calculate_win_universe_count(p1_start_pos: usize, p2_start_pos: usize, win_score: usize) -> u64 {

    0

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn win_univ_count() {
        let result = calculate_win_universe_count(4, 8, 21);
        assert_eq!(result, 444356092776315);
    }
}
