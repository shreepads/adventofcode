// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub mod graph;

use graph::Graph;
use std::fs;

pub fn calculate_total_paths(file_path: &String) -> u32 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let mut graph = Graph::new();

    load_graph(&mut graph, contents);

    graph.all_paths("start", "end").unwrap().len() as u32
}

fn load_graph(graph: &mut Graph, contents: String) {
    for (_, row) in contents.lines().enumerate() {
        let mut edges = row.split("-");

        let from_edge = edges.next().unwrap();
        let to_edge = edges.next().unwrap();

        graph.add_edge(from_edge, to_edge, false);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day12_2_test() {
        let result = calculate_total_paths(&String::from("../resources/tests/day12-2-testdata.txt"));
        assert_eq!(result, 3509);
    }
}
