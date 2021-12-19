// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum NodeType {
    Start,
    Big,
    Small,
    End,
}

#[derive(Debug, Clone)]
pub struct Node {
    linked_nodes: HashSet<String>,
    visited: bool,
    node_type: NodeType,
    node_name: String,
}

impl Node {
    pub fn new(name: String) -> Node {
        
        let node_type: NodeType;
        
        if name == "start" {
            node_type = NodeType::Start;
        } else if name == "end" {
            node_type = NodeType::End;
        } else if name.to_lowercase() == name {
            node_type = NodeType::Small;
        } else {
            node_type = NodeType::Big;
        }
        
        Node {
            linked_nodes: HashSet::new(),
            visited: false,
            node_type: node_type,
            node_name: name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from_node_name: &str, to_node_name: &str, directed: bool) {
        if !self.nodes.contains_key(from_node_name) {
            self.nodes.insert(from_node_name.to_string(), Node::new(from_node_name.to_string()));
        }

        if !self.nodes.contains_key(to_node_name) {
            self.nodes.insert(to_node_name.to_string(), Node::new(to_node_name.to_string()));
        }

        let from_node: &mut Node = self.nodes.get_mut(from_node_name).unwrap();

        from_node.linked_nodes.insert(to_node_name.to_string());

        if !directed {
            let to_node: &mut Node = self.nodes.get_mut(to_node_name).unwrap();

            to_node.linked_nodes.insert(from_node_name.to_string());
        }
    }

    pub fn all_paths(self, from_node_name: &str, to_node_name: &str) -> Option<Vec<Vec<String>>> {

        if !self.nodes.contains_key(from_node_name) {
            return None   // No paths between unknown nodes
        }

        if !self.nodes.contains_key(to_node_name) {
            return None   // No paths between unknown nodes
        }

        // Count of visited nodes
        let mut visited_counts : HashMap<String, u8> = HashMap::new();
        for node_name in self.nodes.keys() {
            visited_counts.insert(node_name.to_string(), 0);
        }

        let mut paths: Vec<Vec<String>> = Vec::new();
        let mut current_path: Vec<String> = Vec::new();

        self.find_all_paths(from_node_name, to_node_name, &mut visited_counts, &mut current_path, &mut paths);

        Some(paths)

    }

    fn find_all_paths(self, from_node_name: &str, to_node_name: &str, 
        visited_counts: &mut HashMap<String, u8>,
        current_path: &mut Vec<String>, 
        paths: &mut Vec<Vec<String>>) {

    }
}