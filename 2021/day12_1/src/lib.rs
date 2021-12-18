// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod graph;

use std::fs;
use graph::Graph;

pub fn calculate_total_paths(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect(&format!("Something went wrong reading the file {}", file_path));

    let mut graph = Graph::new();

    load_graph(&mut graph, contents);

    0
}

fn load_graph(graph: &mut Graph, contents: String) {
    for (row_no, row) in contents.lines().enumerate() {

        let mut edges = row.split("-");

        let from_edge = edges.next().unwrap();
        let to_edge = edges.next().unwrap();

        graph.add_edge(from_edge, to_edge, false);
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
