// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use day15_1::graph::Graph;
use std::fs;

pub fn calculate_least_risk_path(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut grid: [[u32; 502]; 502] = [[u32::MAX; 502]; 502];

    load_grid(&mut grid, contents);

    let mut graph = Graph::new();

    load_graph(&mut graph, &grid);

    graph.shortest_path_weight(1, 250000).unwrap()
}

fn load_graph(graph: &mut Graph, grid: &[[u32; 502]; 502]) {
    for row_no in 1..=500 {
        for col_no in 1..=500 {
            let multiplier = 500;
            let from_node_id = col_no + (row_no - 1) * multiplier;

            // check above
            let above_weight: u32 = grid[row_no - 1][col_no];
            if above_weight != u32::MAX {
                graph.add_edge(
                    from_node_id,
                    col_no + (row_no - 2) * multiplier,
                    above_weight,
                );
            }

            // check below
            let below_weight: u32 = grid[row_no + 1][col_no];
            if below_weight != u32::MAX {
                graph.add_edge(from_node_id, col_no + (row_no) * multiplier, below_weight);
            }

            // check right
            let right_weight: u32 = grid[row_no][col_no + 1];
            if right_weight != u32::MAX {
                graph.add_edge(
                    from_node_id,
                    col_no + 1 + (row_no - 1) * multiplier,
                    right_weight,
                );
            }

            // check left
            let left_weight: u32 = grid[row_no][col_no - 1];
            if left_weight != u32::MAX {
                graph.add_edge(
                    from_node_id,
                    col_no - 1 + (row_no - 1) * multiplier,
                    left_weight,
                );
            }
        }
    }
}

fn load_grid(grid: &mut [[u32; 502]; 502], contents: String) {
    for (row_no, line_str) in contents.lines().enumerate() {
        for (col_no, digit_chr) in line_str.chars().enumerate() {
            let digit: u32 = digit_chr.to_digit(10).unwrap().try_into().unwrap();

            // Fill all 25 frames with given digit
            for frame_row in 0..5 {
                for frame_col in 0..5 {
                    let point_row_no = row_no + 1 + (frame_row * 100);
                    let point_col_no = col_no + 1 + (frame_col * 100);
                    let point_digit = ((digit + frame_row as u32 + frame_col as u32 - 1) % 9) + 1;

                    grid[point_row_no][point_col_no] = point_digit;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day15_2_works() {
        let result = calculate_least_risk_path(&String::from("../resources/day15-1-input.txt"));
        assert_eq!(result, 3022);
    }
}
