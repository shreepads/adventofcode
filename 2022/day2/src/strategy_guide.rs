// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use crate::moves::Move;

#[derive(Debug)]
pub struct StrategyGuide {
    moves: Vec<(Move, Move)>,
}

impl StrategyGuide {
    pub fn new(file_contents: &String) -> StrategyGuide {
        let mut moves: Vec<(Move, Move)> = Vec::new();

        for line in file_contents.lines() {
            moves.push(Self::get_moves_from_guide_line(line))
        }

        StrategyGuide { moves }
    }

    pub fn correct_new(file_contents: &String) -> StrategyGuide {
        let mut moves: Vec<(Move, Move)> = Vec::new();

        for line in file_contents.lines() {
            moves.push(Self::get_correct_moves_from_guide_line(line))
        }

        StrategyGuide { moves }
    }

    pub fn get_total_score(&self) -> u64 {
        let mut total_score = 0;

        for moves in self.moves.iter() {
            match moves {
                // Draws
                (Move::Rock, Move::Rock) => total_score += 3 + 1, // Draw + Rock
                (Move::Paper, Move::Paper) => total_score += 3 + 2, // Draw + Paper
                (Move::Scissors, Move::Scissors) => total_score += 3 + 3, // Draw + Scissors

                // Wins
                (Move::Scissors, Move::Rock) => total_score += 6 + 1, // Win + Rock
                (Move::Rock, Move::Paper) => total_score += 6 + 2,    // Win + Paper
                (Move::Paper, Move::Scissors) => total_score += 6 + 3, // Win + Scissors

                // Losses
                (Move::Paper, Move::Rock) => total_score += 0 + 1, // Loss + Rock
                (Move::Scissors, Move::Paper) => total_score += 0 + 2, // Loss + Paper
                (Move::Rock, Move::Scissors) => total_score += 0 + 3, // Loss + Scissors

                _ => {}
            }
        }

        total_score
    }

    // Can't create a const HashMap to convert strategy guide chars to Moves
    // so use this helper function instead for whole line

    fn get_moves_from_guide_line(guide_line: &str) -> (Move, Move) {
        let mut first_move = Move::Invalid;
        let mut second_move = Move::Invalid;

        let mut chars = guide_line.chars();

        match chars.next() {
            Some('A') => first_move = Move::Rock,
            Some('B') => first_move = Move::Paper,
            Some('C') => first_move = Move::Scissors,
            _ => {}
        }

        _ = chars.next();

        match chars.next() {
            Some('X') => second_move = Move::Rock,
            Some('Y') => second_move = Move::Paper,
            Some('Z') => second_move = Move::Scissors,
            _ => {}
        }

        (first_move, second_move)
    }

    fn get_correct_moves_from_guide_line(guide_line: &str) -> (Move, Move) {
        let mut first_move = Move::Invalid;
        let mut second_move = Move::Invalid;

        let mut chars = guide_line.chars();

        match chars.next() {
            Some('A') => first_move = Move::Rock,
            Some('B') => first_move = Move::Paper,
            Some('C') => first_move = Move::Scissors,
            _ => {}
        }

        _ = chars.next();

        match chars.next() {
            Some('X') => {
                // Lose
                match first_move {
                    Move::Rock => second_move = Move::Scissors,
                    Move::Paper => second_move = Move::Rock,
                    Move::Scissors => second_move = Move::Paper,
                    _ => {}
                }
            }
            Some('Y') => {
                // Draw
                match first_move {
                    Move::Rock => second_move = Move::Rock,
                    Move::Paper => second_move = Move::Paper,
                    Move::Scissors => second_move = Move::Scissors,
                    _ => {}
                }
            }
            Some('Z') => {
                // Win
                match first_move {
                    Move::Rock => second_move = Move::Paper,
                    Move::Paper => second_move = Move::Scissors,
                    Move::Scissors => second_move = Move::Rock,
                    _ => {}
                }
            }
            _ => {}
        }

        (first_move, second_move)
    }
} // impl StrategyGuide
