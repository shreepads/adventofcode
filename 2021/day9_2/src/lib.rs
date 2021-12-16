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

    //println!("Grid: \n {:?}", grid);

    // Convert grid in graph by edges

    let mut graph = Graph::new();

    load_graph(&mut graph, &grid);

    /*
    println!("Graph nodes:");
    println!("Graph node 1: {:?}", graph.nodes.get(&1).unwrap());
    println!("Graph node 2: {:?}", graph.nodes.get(&2).unwrap());
    println!("Graph node 99: {:?}", graph.nodes.get(&99).unwrap());
    println!("Graph node 100: {:?}", graph.nodes.get(&100).unwrap());
    //println!("Graph node 101: {:?}", graph.nodes.get(&101).unwrap());
    println!("Graph node 102: {:?}", graph.nodes.get(&102).unwrap());
    println!("Graph node 105: {:?}", graph.nodes.get(&105).unwrap());
    println!("Graph node 199: {:?}", graph.nodes.get(&199).unwrap());
    //println!("Graph node 200: {:?}", graph.nodes.get(&200).unwrap());
    //println!("Graph node 9801: {:?}", graph.nodes.get(&9801).unwrap());
    println!("Graph node 9802: {:?}", graph.nodes.get(&9802).unwrap());
    //println!("Graph node 9899: {:?}", graph.nodes.get(&9899).unwrap());
    println!("Graph node 9900: {:?}", graph.nodes.get(&9900).unwrap());
    println!("Graph node 9901: {:?}", graph.nodes.get(&9901).unwrap());
    println!("Graph node 9902: {:?}", graph.nodes.get(&9902).unwrap());
    //println!("Graph node 9999: {:?}", graph.nodes.get(&9999).unwrap());
    //println!("Graph node 10000: {:?}", graph.nodes.get(&10000).unwrap());
    */

    0u32

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



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
