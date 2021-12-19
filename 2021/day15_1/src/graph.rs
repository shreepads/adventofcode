// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Edge {
    to_node_id: usize,
    weight: u32,
}


#[derive(Debug, Clone)]
pub struct Graph {
    nodes: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn shortest_path_weight(&self, start_node_id: usize, end_node_id: usize) -> Option<u32> {

        Some(0)

    }

    pub fn add_edge(&mut self, from_node_id: usize, to_node_id: usize, weight: u32) {

        if !self.nodes.contains_key(&from_node_id) {
            self.nodes.insert(from_node_id, Vec::new());
        }

        if !self.nodes.contains_key(&to_node_id) {
            self.nodes.insert(to_node_id, Vec::new());
        }

        let from_node_edges = self.nodes.get_mut(&from_node_id).unwrap();

        from_node_edges.push(
            Edge {
                to_node_id: to_node_id,
                weight: weight,
            }
        );
    }

    pub fn print(&self) {
        println!("Node {} edges: {:?}", 1, self.nodes.get(&1).unwrap());
        println!("Node {} edges: {:?}", 2, self.nodes.get(&2).unwrap());
        println!("Node {} edges: {:?}", 100, self.nodes.get(&100).unwrap());
        println!("Node {} edges: {:?}", 101, self.nodes.get(&101).unwrap());
        println!("Node {} edges: {:?}", 102, self.nodes.get(&102).unwrap());
        println!("Node {} edges: {:?}", 200, self.nodes.get(&200).unwrap());
        println!("Node {} edges: {:?}", 10000, self.nodes.get(&10000).unwrap());
    }
}
