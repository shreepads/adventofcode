// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod moves;
mod strategy_guide;

use std::fs;

use strategy_guide::StrategyGuide;

pub fn strategy_guide_score(file_path: &String) -> u64 {

    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let strat_guide = StrategyGuide::new(&file_contents);

    strat_guide.get_total_score()

}

pub fn correct_strategy_guide_score(file_path: &String) -> u64 {

    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");
    
    let strat_guide = StrategyGuide::correct_new(&file_contents);

    strat_guide.get_total_score()

}




