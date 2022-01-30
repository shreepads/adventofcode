// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod burrow;
mod path;

//use std::collections::HashMap;
use ahash::AHashMap;
use std::fs;

use day15_1::graph::Graph;

use burrow::AmphiType;
use burrow::BurrowState;
use burrow::PositionState;
use burrow::MAX_POS;

pub fn calculate_min_energy(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let start_state = load_data(contents);
    let end_state = BurrowState::new_end();

    let mut graph = Graph::new();

    let (start_id, end_id) = load_graph(&mut graph, start_state, end_state);

    graph.shortest_path_weight(start_id, end_id).unwrap()
}

fn load_graph(
    graph: &mut Graph,
    start_state: BurrowState,
    end_state: BurrowState,
) -> (usize, usize) {
    let mut states_vec: Vec<BurrowState> = Vec::new();
    let mut states_hmap: AHashMap<BurrowState, usize> = AHashMap::new();

    // Insert start and end position
    states_vec.push(end_state);
    let end_id = 0;
    states_hmap.insert(end_state, end_id);

    states_vec.push(start_state);
    let start_id = 1;
    states_hmap.insert(start_state, start_id);

    // Insert start posn in to do list
    let mut todo_states_vec: Vec<BurrowState> = Vec::new();
    todo_states_vec.push(start_state);

    // Generate next states and insert in graph till no more

    while let Some(current_state) = todo_states_vec.pop() {
        // add current state to collection if not present and get id
        let current_id: usize = if states_hmap.contains_key(&current_state) {
            *states_hmap.get(&current_state).unwrap()
        } else {
            states_vec.push(current_state);
            states_hmap.insert(current_state, states_vec.len() - 1);
            states_vec.len() - 1
        };

        let next_states = current_state.next_states();

        for (energy, next_state) in next_states.iter() {
            // if next state not seen before, add to to-do list
            if !states_hmap.contains_key(next_state) {
                todo_states_vec.push(*next_state);
            }

            // add next state to collection if not present and get id
            let next_state_id: usize = if states_hmap.contains_key(&next_state) {
                *states_hmap.get(&next_state).unwrap()
            } else {
                states_vec.push(*next_state);
                states_hmap.insert(*next_state, states_vec.len() - 1);
                states_vec.len() - 1
            };

            // add edge to graph from current_state to next_state
            graph.add_edge(current_id, next_state_id, *energy as u32);
        }
    }

    (start_id, end_id)
}

fn load_data(contents: String) -> BurrowState {
    use AmphiType::*;
    use PositionState::*;

    let mut posns: [PositionState; MAX_POS] = [Empty; MAX_POS];

    let mut lines = contents.lines();
    lines.next();
    lines.next();

    for (row_no, line) in lines.enumerate() {
        if line.contains("######") {
            break;
        }

        let mut chars = line.chars();

        let mut cols: [char; 4] = [' '; 4];

        cols[0] = chars.nth(3).unwrap();
        cols[1] = chars.nth(1).unwrap();
        cols[2] = chars.nth(1).unwrap();
        cols[3] = chars.nth(1).unwrap();

        for (col_no, col) in cols.iter().enumerate() {
            let posn_idx = 11 + row_no + (col_no * 4);

            posns[posn_idx] = match col {
                'A' => Occupied(A),
                'B' => Occupied(B),
                'C' => Occupied(C),
                'D' => Occupied(D),
                _ => {
                    println!("Invalid char {} at col no {} in line {}", col, col_no, line);
                    Empty
                }
            }
        }
    }

    BurrowState { positions: posns }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn burrow_test() {
        let result = calculate_min_energy(&String::from("../resources/tests/day23-2-testdata.txt"));
        assert_eq!(result, 44169);
    }
}
