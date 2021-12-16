// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod graph;

use std::fs;
use graph::Graph;

pub fn top3_basins_product(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut grid : Vec<Vec<u8>> = Vec::new();

    load_grid(&mut grid, contents);

    // Convert grid in graph by edges

    let mut graph = Graph::new();

    load_graph(&mut graph, &grid);

    // Find basin sizes

    let mut basin_sizes : Vec<u32> = Vec::new();

    find_basin_sizes(&mut graph, &mut basin_sizes);

    basin_sizes.sort_unstable();

    basin_sizes.iter().skip(basin_sizes.len() - 3).product()

}


fn load_grid(grid: &mut Vec<Vec<u8>>, contents: String) {

    for (line_no, line_str) in contents.lines().enumerate() {

        if line_no == 0 {
            grid.push(vec![9; line_str.len() + 2]);
        }

        let mut line : Vec<u8> = Vec::new();

        line.push(9);

        for digit in line_str.chars() {
            line.push(digit.to_digit(10).unwrap().try_into().unwrap());
        }

        line.push(9);

        grid.push(line);

    }

    grid.push(vec![9; grid[0].len()]);

}

fn load_graph(graph: &mut Graph, grid: &Vec<Vec<u8>>) {

    for (row_no, row) in grid.iter().enumerate() {

        if row_no == 0  ||  row_no == grid.len()-1 {
            continue;
        }

        for (col_no, height) in row.iter().enumerate() {

            if col_no == 0  ||  col_no == row.len()-1 {
                continue;
            }

            if *height == 9 {
                continue;
            }

            // Assume a single disconnected point cannot be one of the 
            // 3 biggest basins
            add_edges(graph, grid, row_no, col_no);
        }
    }

}


fn add_edges(graph: &mut Graph, grid: &Vec<Vec<u8>>, row_no: usize, col_no: usize) {

    let multiplier = grid[0].len() - 2;
    let from_node = col_no + (row_no - 1) * multiplier;

    // check above
    if grid[row_no - 1][col_no] != 9 {
        graph.add_edge(from_node, col_no + (row_no - 2) * multiplier, false);
    }

    // check below
    if grid[row_no + 1][col_no] != 9 {
        graph.add_edge(from_node, col_no + (row_no) * multiplier, false);
    }

    // check right
    if grid[row_no][col_no + 1] != 9 {
        graph.add_edge(from_node, col_no + 1 + (row_no - 1) * multiplier, false);
    }

    // check left
    if grid[row_no][col_no - 1] != 9 {
        graph.add_edge(from_node, col_no - 1 + (row_no - 1) * multiplier, false);
    }

}

fn find_basin_sizes(graph: &mut Graph, basin_sizes: &mut Vec<u32>) {

    for (node_id, _) in graph.nodes.clone().iter() {
        let basin_size = graph.connected_size(*node_id);
        if basin_size != 0 {
            basin_sizes.push(basin_size);
        }
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
