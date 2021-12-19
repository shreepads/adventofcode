// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod graph;

use std::fs;
use graph::Graph;

pub fn calculate_least_risk_path(file_path: &String) -> u32 {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut grid: [ [u32; 102] ; 102] = [ [u32::MAX; 102] ; 102];

    load_grid(&mut grid, contents);

    let mut graph = Graph::new();

    load_graph(&mut graph, &grid);

    graph.shortest_path_weight(1, 10000).unwrap()

}

fn load_graph(graph: &mut Graph, grid: &[ [u32; 102] ; 102]) {

    for row_no in 1..=100 {
        for col_no in 1..=100 {
            let multiplier = 100;
            let from_node_id = col_no + (row_no - 1) * multiplier;

            // check above
            let above_weight: u32 = grid[row_no - 1][col_no];
            if above_weight != u32::MAX {
                graph.add_edge(from_node_id, col_no + (row_no - 2) * multiplier, above_weight);
            }

            // check below
            let below_weight: u32 = grid[row_no + 1][col_no];
            if below_weight != u32::MAX {
                graph.add_edge(from_node_id, col_no + (row_no) * multiplier, below_weight);
            }

            // check right
            let right_weight: u32 = grid[row_no][col_no + 1];
            if right_weight != u32::MAX {
                graph.add_edge(from_node_id, col_no + 1 + (row_no - 1) * multiplier, right_weight);
            }

            // check left
            let left_weight: u32 = grid[row_no][col_no - 1];
            if left_weight != u32::MAX {
                graph.add_edge(from_node_id, col_no - 1 + (row_no - 1) * multiplier, left_weight);
            }

        }
    }
}

fn load_grid(grid: &mut[ [u32; 102] ; 102], contents: String) {
    for (row_no, line_str) in contents.lines().enumerate() {
        for (col_no, digit_chr) in line_str.chars().enumerate() {
            let digit: u32 = digit_chr.to_digit(10).unwrap().try_into().unwrap();
            grid[row_no + 1][col_no + 1] = digit;
        }
    }
}

fn _print_grid(grid: &[ [u32; 102] ; 102]) {
    for row in grid.iter() {
        for digit in row.iter() {
            if *digit == u32::MAX {
                print!("*");
            } else {
                print!("{}", digit);
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
