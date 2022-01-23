// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::{thread, time};

pub const MAX_POS: usize = 10;
pub const MAX_SCORE: usize = 100; // needed if we're waiting for ALL player x's to win

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct GameState {
    p1_pos: usize,
    p1_score: usize,

    p2_pos: usize,
    p2_score: usize,

    p1_plays_next: bool,
}

pub fn calculate_win_universe_count(
    p1_start_pos: usize,
    p2_start_pos: usize,
    win_score: usize,
) -> u64 {
    let init_game_state = GameState {
        p1_pos: p1_start_pos,
        p1_score: 0,
        p2_pos: p2_start_pos,
        p2_score: 0,
        p1_plays_next: true,
    };

    // Setup game state
    let mut games_state_counts: HashMap<GameState, u64> = HashMap::new();
    games_state_counts.insert(init_game_state, 1);

    // Setup map for distribution of throws
    let throw_map: HashMap<usize, u64> = HashMap::from([
        (3, 1), // Total of 3 throws = 3 in 1 universe
        (4, 3), // Total of 3 throws = 4 in 3 universes
        (5, 6), // Total of 3 throws = 5 in 6 universes
        (6, 7), // Total of 3 throws = 6 in 7 universes
        (7, 6), // Total of 3 throws = 7 in 6 universes
        (8, 3), // Total of 3 throws = 8 in 3 universes
        (9, 1), // Total of 3 throws = 9 in 1 universe
    ]);

    let mut round = 1;

    loop {
        move_pawns(&mut games_state_counts, round, &throw_map, win_score);

        if all_games_complete(&games_state_counts, win_score) {
            break;
        }

        round += 1;
    }

    let (player1_wins, player2_wins) = count_win_universes(&games_state_counts, win_score);

    if player1_wins > player2_wins {
        player1_wins
    } else {
        player2_wins
    }
}

fn count_win_universes(
    games_state_counts: &HashMap<GameState, u64>,
    win_score: usize,
) -> (u64, u64) {
    let mut player1_win_count = 0u64;
    let mut player2_win_count = 0u64;

    for (gamestate, state_count) in games_state_counts.iter() {
        if gamestate.p1_score >= win_score {
            player1_win_count += state_count;
        } else if gamestate.p2_score >= win_score {
            player2_win_count += state_count;
        } else {
            println!("Something's wrong with gamestate: {:?}", gamestate);
        }
    }

    (player1_win_count, player2_win_count)
}

fn all_games_complete(games_state_counts: &HashMap<GameState, u64>, win_score: usize) -> bool {
    for gamestate in games_state_counts.keys() {
        if gamestate.p1_score < win_score && gamestate.p2_score < win_score {
            return false;
        }
    }

    true
}

fn _print_game_states(games_state_counts: &HashMap<GameState, u64>) {
    println!("Game states: {:?}", games_state_counts);

    thread::sleep(time::Duration::from_secs(1));
}

fn move_pawns(
    games_state_counts: &mut HashMap<GameState, u64>,
    round: usize,
    throw_map: &HashMap<usize, u64>,
    win_score: usize,
) {
    let mut new_games_state_counts: HashMap<GameState, u64> = HashMap::new();

    for (gamestate, state_count) in games_state_counts.iter() {
        if gamestate.p1_score >= win_score || gamestate.p2_score >= win_score {
            let upd_state_count = new_games_state_counts.entry(*gamestate).or_insert(0);
            *upd_state_count += state_count;
            continue; // game complete, nothing to do here
        }

        if round % 2 == 1 && !gamestate.p1_plays_next {
            let upd_state_count = new_games_state_counts.entry(*gamestate).or_insert(0);
            *upd_state_count += state_count;
            continue; // odd rounds to be played by p1
        }

        if round % 2 == 0 && gamestate.p1_plays_next {
            let upd_state_count = new_games_state_counts.entry(*gamestate).or_insert(0);
            *upd_state_count += state_count;
            continue; // even rounds to be played by p2
        }

        if gamestate.p1_plays_next {
            //move p1
            for (throw_total, throw_count) in throw_map.iter() {
                let new_p1_pos = ((gamestate.p1_pos + throw_total - 1) % 10) + 1;
                let new_p1_score = gamestate.p1_score + new_p1_pos;
                let new_state_count = state_count * throw_count;

                let new_state = GameState {
                    p1_pos: new_p1_pos,
                    p1_score: new_p1_score,
                    p2_pos: gamestate.p2_pos,
                    p2_score: gamestate.p2_score,
                    p1_plays_next: false,
                };

                let upd_state_count = new_games_state_counts.entry(new_state).or_insert(0);
                *upd_state_count += new_state_count;
            }
        } else {
            // move p2
            for (throw_total, throw_count) in throw_map.iter() {
                let new_p2_pos = ((gamestate.p2_pos + throw_total - 1) % 10) + 1;
                let new_p2_score = gamestate.p2_score + new_p2_pos;
                let new_state_count = state_count * throw_count;

                let new_state = GameState {
                    p1_pos: gamestate.p1_pos,
                    p1_score: gamestate.p1_score,
                    p2_pos: new_p2_pos,
                    p2_score: new_p2_score,
                    p1_plays_next: true,
                };

                let upd_state_count = new_games_state_counts.entry(new_state).or_insert(0);
                *upd_state_count += new_state_count;
            }
        }
    }

    *games_state_counts = new_games_state_counts;
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
