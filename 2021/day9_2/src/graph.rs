// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    linked_nodes: Vec<u32>,
    visited: bool,
}

impl Node {
    pub fn new() -> Node {
        Node {
            linked_nodes: Vec::new(),
            visited: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: HashMap<u32, Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }
}
