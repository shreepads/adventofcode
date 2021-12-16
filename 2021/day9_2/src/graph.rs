// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Node {
    linked_nodes: HashSet<usize>,
    visited: bool,
}

impl Node {
    pub fn new() -> Node {
        Node {
            linked_nodes: HashSet::new(),
            visited: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from_node_id: usize, to_node_id: usize, directed: bool) {

        if !self.nodes.contains_key(&from_node_id) {
            self.nodes.insert(from_node_id, Node::new());
        }

        if !self.nodes.contains_key(&to_node_id) {
            self.nodes.insert(to_node_id, Node::new());
        }

        let from_node: &mut Node = self.nodes.get_mut(&from_node_id).unwrap();

        from_node.linked_nodes.insert(to_node_id);

        if !directed {

            let to_node: &mut Node = self.nodes.get_mut(&to_node_id).unwrap();

            to_node.linked_nodes.insert(from_node_id);
    
        }

    }
}
