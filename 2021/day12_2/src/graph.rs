// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use ahash::AHashMap;
use ahash::AHashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Start,
    Big,
    Small,
    End,
}

#[derive(Debug, Clone)]
pub struct Node {
    linked_nodes: AHashSet<String>,
    node_type: NodeType,
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
            linked_nodes: AHashSet::new(),
            node_type: node_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: AHashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: AHashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from_node_name: &str, to_node_name: &str, directed: bool) {
        if !self.nodes.contains_key(from_node_name) {
            self.nodes.insert(
                from_node_name.to_string(),
                Node::new(from_node_name.to_string()),
            );
        }

        if !self.nodes.contains_key(to_node_name) {
            self.nodes.insert(
                to_node_name.to_string(),
                Node::new(to_node_name.to_string()),
            );
        }

        let from_node: &mut Node = self.nodes.get_mut(from_node_name).unwrap();

        from_node.linked_nodes.insert(to_node_name.to_string());

        if !directed {
            let to_node: &mut Node = self.nodes.get_mut(to_node_name).unwrap();

            to_node.linked_nodes.insert(from_node_name.to_string());
        }
    }

    pub fn all_paths(self, from_node_name: &str, to_node_name: &str) -> usize {
        if !self.nodes.contains_key(from_node_name) {
            return 0; // No paths between unknown nodes
        }

        if !self.nodes.contains_key(to_node_name) {
            return 0; // No paths between unknown nodes
        }

        let mut paths: AHashSet<String> = AHashSet::new();

        // Count of visited nodes with each given Small visited twice
        for (small_node_name, _) in self
            .nodes
            .iter()
            .filter(|(_, node)| node.node_type == NodeType::Small)
        {
            let mut visited_counts: AHashMap<String, u8> = AHashMap::new();
            for node_name in self.nodes.keys() {
                visited_counts.insert(node_name.to_string(), 0);
            }

            let mut current_path: Vec<String> = Vec::new();

            self.find_all_paths(
                from_node_name,
                to_node_name,
                small_node_name,
                &mut visited_counts,
                &mut current_path,
                &mut paths,
            );
        }

        //paths.sort_unstable();
        //paths.dedup();

        paths.len()
    }

    fn find_all_paths(
        &self,
        from_node_name: &str,
        to_node_name: &str,
        small_node_name: &str,
        visited_counts: &mut AHashMap<String, u8>,
        current_path: &mut Vec<String>,
        paths: &mut AHashSet<String>,
    ) {
        let from_node = self.nodes.get(from_node_name).unwrap();
        // deref and re-insert to prevent borrow problems
        let from_node_visit_count = *visited_counts.get(from_node_name).unwrap();

        if from_node.node_type == NodeType::Start && from_node_visit_count > 0 {
            return;
        }

        if from_node.node_type == NodeType::End && from_node_visit_count > 0 {
            return;
        }

        if from_node.node_type == NodeType::Small {
            // Special small node can be visited twice others once
            if from_node_name == small_node_name && from_node_visit_count > 1 {
                return;
            }

            if from_node_name != small_node_name && from_node_visit_count > 0 {
                return;
            }
        }

        if from_node_name == to_node_name {
            // reached the end, insert current path as String in paths
            let current_path_str = current_path
                .iter()
                .fold(String::new(), |acc, x| format!("{}-{}", acc, x));
            paths.insert(current_path_str);
            return;
        }

        visited_counts.insert(from_node_name.to_string(), from_node_visit_count + 1);
        current_path.push(from_node_name.to_string());

        // visit all linked nodes recursively
        for linked_node_name in from_node.linked_nodes.clone().iter() {
            self.find_all_paths(
                linked_node_name,
                to_node_name,
                small_node_name,
                visited_counts,
                current_path,
                paths,
            );
        }

        // Clear from_node from path, visit count
        visited_counts.insert(from_node_name.to_string(), from_node_visit_count);
        current_path.pop();

        return;
    }
}
