// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

// Hold state on BinaryHeap priority queue with min ordering
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node_id: usize,
    total_weight: u32,
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement min ordering for State to pick lowest total weight
// If weights equal pick lower node id
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .total_weight
            .cmp(&self.total_weight)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

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
        // Initialise distance from start node to all nodes
        let mut distances: HashMap<usize, u32> = HashMap::new();
        for node_id in self.nodes.keys() {
            distances.insert(*node_id, u32::MAX);
        }
        distances.insert(start_node_id, 0);

        // Initialise priority queue with start position and 0 total weight
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(State {
            node_id: start_node_id,
            total_weight: 0,
        });

        // Process lowest cost node from priority queue
        while let Some(min_state) = priority_queue.pop() {
            let min_node_id = min_state.node_id;
            let min_total_weight = min_state.total_weight;

            // Check if reached end
            if min_node_id == end_node_id {
                return Some(min_total_weight);
            }

            // Discard if lowest cost route to this node exists
            if min_total_weight > *distances.get(&min_node_id).unwrap() {
                continue;
            }

            // For each node reachable from current node, see if lower route
            // via this node exists
            for edge in self.nodes.get(&min_node_id).unwrap().iter() {
                let to_node_id = edge.to_node_id;
                let to_node_total_weight = min_total_weight + edge.weight;

                // If shorter via this node, add to state
                if to_node_total_weight < *distances.get(&to_node_id).unwrap() {
                    priority_queue.push(State {
                        node_id: to_node_id,
                        total_weight: to_node_total_weight,
                    });

                    // Update least distances map
                    distances.insert(to_node_id, to_node_total_weight);
                }
            }
        }

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

        from_node_edges.push(Edge {
            to_node_id: to_node_id,
            weight: weight,
        });
    }

    pub fn _print(&self) {
        println!("Node {} edges: {:?}", 1, self.nodes.get(&1).unwrap());
        println!("Node {} edges: {:?}", 2, self.nodes.get(&2).unwrap());
        println!("Node {} edges: {:?}", 100, self.nodes.get(&100).unwrap());
        println!("Node {} edges: {:?}", 101, self.nodes.get(&101).unwrap());
        println!("Node {} edges: {:?}", 102, self.nodes.get(&102).unwrap());
        println!("Node {} edges: {:?}", 200, self.nodes.get(&200).unwrap());
        println!(
            "Node {} edges: {:?}",
            10000,
            self.nodes.get(&10000).unwrap()
        );
    }
}
