// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::{thread, time};

pub const MAX_POS: usize = 10;
pub const MAX_SCORE: usize = 100;   // needed if we're waiting for ALL player x's to win

pub fn calculate_win_universe_count(p1_start_pos: usize, 
    p2_start_pos: usize, win_score: usize) -> u64 {

    let mut player1_states : [[u64; MAX_SCORE]; MAX_POS] = [[0; MAX_SCORE]; MAX_POS];
    let mut player2_states : [[u64; MAX_SCORE]; MAX_POS] = [[0; MAX_SCORE]; MAX_POS];

    // Set player start posn at zero score
    player1_states[p1_start_pos - 1][0] = 1;
    player2_states[p2_start_pos - 1][0] = 1;

    // Setup map for distribution of throws
    let throw_map: HashMap<usize, u64> = HashMap::from([
        (3, 1),         // Total of 3 throws = 3 in 1 universe
        (4, 3),         // Total of 3 throws = 4 in 3 universes
        (5, 6),         // Total of 3 throws = 5 in 6 universes
        (6, 7),         // Total of 3 throws = 6 in 7 universes
        (7, 6),         // Total of 3 throws = 7 in 6 universes
        (8, 3),         // Total of 3 throws = 8 in 3 universes
        (9, 1),         // Total of 3 throws = 9 in 1 universe
    ]);

    let mut round = 1;
    
    loop {
        println!("Round: {}", round);
        println!("*********");
        print_states(&player1_states, &player2_states);

        move_pawns(&mut player1_states, &throw_map);

        if won_all_universes(&player1_states, win_score) {
            println!("**** PLAYER 1 WINS ****");
            break;
        }

        move_pawns(&mut player2_states, &throw_map);

        if won_all_universes(&player2_states, win_score) {
            println!("**** PLAYER 2 WINS ****");
            break;
        }

        
        round += 1;
        
    }

    let player1_wins = count_win_universes(&player1_states, win_score);
    let player2_wins = count_win_universes(&player2_states, win_score);

    println!("Player 1 wins in {} univs, Player 2 in {}", player1_wins, player2_wins);
    
    if player1_wins > player2_wins {
        player1_wins
    } else {
        player2_wins
    }

}

fn count_win_universes(player_states: &[[u64; MAX_SCORE]; MAX_POS], win_score: usize) -> u64 {
    
    let mut win_count = 0u64;

    for (posn, posn_scores) in player_states.iter().enumerate() {
        for (score, score_count) in posn_scores.iter().enumerate() {
            if score < win_score {
                continue;
            }

            win_count += score_count;
        }
    }

    win_count
}

fn won_all_universes(player_states: &[[u64; MAX_SCORE]; MAX_POS], win_score: usize) -> bool {

    for (posn, posn_scores) in player_states.iter().enumerate() {
        for (score, score_count) in posn_scores.iter().enumerate() {
            if score < win_score  &&  *score_count > 0 {
                return false;
            }
        }
    }

    true

}

fn print_states(player1_states: &[[u64; MAX_SCORE]; MAX_POS],
    player2_states: &[[u64; MAX_SCORE]; MAX_POS]) {

    println!("Player 1:");
    for (posn, posn_scores) in player1_states.iter().enumerate() {
        print!("Posn {}: ", posn + 1);
        for (score, score_count) in posn_scores.iter().enumerate() {
            if *score_count != 0 {
                print!("({}, {}); ", score, score_count);
            }
        }
        println!("");
    }
    println!("");

    println!("Player 2:");
    for (posn, posn_scores) in player2_states.iter().enumerate() {
        print!("Posn {}: ", posn + 1);
        for (score, score_count) in posn_scores.iter().enumerate() {
            if *score_count != 0 {
                print!("({}, {}); ", score, score_count);
            }
        }
        println!("");
    }
    println!("");

    thread::sleep(time::Duration::from_secs(1));

}


fn move_pawns(player_states: &mut [[u64; MAX_SCORE]; MAX_POS], throw_map: &HashMap<usize, u64>) {

    let mut new_player_states : [[u64; MAX_SCORE]; MAX_POS] = [[0; MAX_SCORE]; MAX_POS];

    // Update player states
    for (posn, posn_scores) in player_states.iter().enumerate() {
        for (score, score_count) in posn_scores.iter().enumerate() {
            // For each position, score count, apply all throws
            if *score_count == 0 {continue;}  // do nothing if no pawns in this position/ score

            for (throw_total, throw_count) in throw_map.iter() {
                let new_posn = (posn + throw_total) % MAX_POS;
                let new_score = score + new_posn + 1;
                let new_score_count = score_count * throw_count;

                new_player_states[new_posn][new_score] += new_score_count;
            }
        }
    }

    *player_states = new_player_states;

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
