// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Node {
    part1_id: Option<usize>,
    part2_id: Option<usize>,
    parent_id: Option<usize>,
    value: Option<u32>,
    left_id: Option<usize>,
    right_id: Option<usize>,
    pair: bool,
    depth: u32,
}

#[derive(Debug, Clone)]
pub struct Number {
    nodes: Vec<Node>,
    rootnode_id: usize,
}


impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum StackElement {
    SfishNodeId(usize),
    FirstOpenBracket,
    OpenBracket,
}

impl Number {
    pub fn new(line: String) -> Number {
        let mut sfish_no = Number {
            nodes: Vec::new(),
            rootnode_id: 0,     // potential error
        };

        let mut stack: Vec<StackElement> = Vec::new();
        let mut depth = 1u32;

        // Assume input snailfish number is reduced i.e. single digits
        for (i, sno_char) in line.chars().enumerate() {
            //println!("Stack: {:?}", stack);
            match sno_char {
                ',' => {},   // ignore ,
                '[' => {
                    if i == 0 {
                        stack.push(StackElement::FirstOpenBracket);
                    } else {
                        stack.push(StackElement::OpenBracket);
                        depth += 1;
                    }
                },
                ']' => {
                    // reduce depth
                    depth -= 1;

                    // pop Numbers and open bracket
                    if let StackElement::SfishNodeId(sno2) = stack.pop().unwrap() {
                        if let StackElement::SfishNodeId(sno1) = stack.pop().unwrap() {
                            // check if first open bracket or not
                            match stack.pop().unwrap() {
                                StackElement::OpenBracket => {
                                    // create new node
                                    let node = Node {
                                        part1_id: Some(sno1),
                                        part2_id: Some(sno2),
                                        parent_id: None,
                                        value: None,
                                        left_id: None,
                                        right_id: None,
                                        pair: true,
                                        depth: depth,                       
                                    };
                                    sfish_no.nodes.push(node);
                                    let node_id = sfish_no.nodes.len() - 1;

                                    // Set this node as parent of sno1 and sno2
                                    sfish_no.nodes[sno1].parent_id = Some(node_id);
                                    sfish_no.nodes[sno2].parent_id = Some(node_id);

                                    // Push back onto stack
                                    stack.push(StackElement::SfishNodeId(node_id));
                                },
                                StackElement::FirstOpenBracket => {
                                    // create root node
                                    let root_node = Node {
                                        part1_id: Some(sno1),
                                        part2_id: Some(sno2),
                                        parent_id: None,
                                        value: None,
                                        left_id: None,
                                        right_id: None,
                                        pair: true,
                                        depth: depth,                       
                                    };
                                    sfish_no.nodes.push(root_node);
                                    let node_id = sfish_no.nodes.len() - 1;

                                    // Set this node as parent of sno1 and sno2
                                    sfish_no.nodes[sno1].parent_id = Some(node_id);
                                    sfish_no.nodes[sno2].parent_id = Some(node_id);

                                    // Set this as the root node
                                    sfish_no.rootnode_id = node_id;
                                },
                                _ => println!("Error couldn't pop open bracket after sno1")
                            };
                        } else {
                            println!("Error couldn't pop sno1 after sno2");
                        }
                    } else {
                        println!("Error couldn't pop sno2 after closing bracket");
                    }
                },
                _   => {
                    // get digit value and push
                    let val: u32 = sno_char.to_digit(10).unwrap();
                    let node = Node {
                        part1_id: None,
                        part2_id: None,
                        parent_id: None,
                        value: Some(val),
                        left_id: None,
                        right_id: None,
                        pair: false,
                        depth: depth,                       
                    };
                    sfish_no.nodes.push(node);
                    stack.push(StackElement::SfishNodeId(sfish_no.nodes.len() - 1));
                },
            }
        }
                    
        sfish_no
    }

    pub fn add(&mut self, sfish_no: &Number) {

        let id_delta = self.nodes.len();

        // move nodes over to self
        for node in sfish_no.nodes.iter() {
            // create new node
            let mut new_node = Node {
                part1_id: None,
                part2_id: None,
                parent_id: None,
                value: None,
                left_id: None,
                right_id: None,
                pair: node.pair,
                depth: node.depth,                       
            };

            // copy fields, adjusting links
            if let Some(part1_id) = node.part1_id {
                new_node.part1_id = Some(part1_id + id_delta);
            }

            if let Some(part2_id) = node.part2_id {
                new_node.part2_id = Some(part2_id + id_delta);
            }

            if let Some(parent_id) = node.parent_id {
                new_node.parent_id = Some(parent_id + id_delta);
            }

            if let Some(value) = node.value {
                new_node.value = Some(value);
            }

            if let Some(left_id) = node.left_id {
                new_node.left_id = Some(left_id + id_delta);
            }

            if let Some(right_id) = node.right_id {
                new_node.right_id = Some(right_id + id_delta);
            }

            // add to self
            self.nodes.push(new_node);

        }

        // create new parent node
        let mut new_rootnode = Node {
            part1_id: Some(self.rootnode_id),
            part2_id: Some(sfish_no.rootnode_id + id_delta),
            parent_id: None,
            value: None,
            left_id: None,
            right_id: None,
            pair: true,
            depth: 1,                       
        };

        self.nodes.push(new_rootnode);
        self.rootnode_id = self.nodes.len() - 1;

    }
    
    pub fn magnitude(&self) -> u32 {
        0
    }

    pub fn to_string(&self) -> String {
        
        let mut sfish_string = String::new();

        self.stringify(&mut sfish_string, self.rootnode_id);

        sfish_string
    }

    fn stringify(&self, sfish_string: &mut String, node_id: usize) {
        
        let node = self.nodes[node_id];
        
        if !node.pair {
            sfish_string.push_str(&node.value.unwrap().to_string());
        } else {
            sfish_string.push('[');
            self.stringify(sfish_string, node.part1_id.unwrap());
            sfish_string.push(',');
            self.stringify(sfish_string, node.part2_id.unwrap());
            sfish_string.push(']');
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_sno_pair() {
        let input = "[1,2]";
        let result = Number::new(input.to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(input, result.to_string());
    }

    #[test]
    fn new_sno_pair_lt() {
        let input = "[[1,2],3]";
        let result = Number::new(input.to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(input, result.to_string());
    }

    #[test]
    fn new_sno_pair_rt() {
        let input = "[9,[8,7]]";
        let result = Number::new(input.to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(input, result.to_string());
    }

    
    #[test]
    fn new_sno_two_pairs() {
        let input = "[[1,9],[8,5]]";
        let result = Number::new(input.to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(input, result.to_string());
    }

    #[test]
    fn add() {
        let mut sno1 = Number::new("[2,3]".to_string());
        let sno2 = Number::new("[4,5]".to_string());
        sno1.add(&sno2);
        println!("Addition: {:#?}", sno1);
        println!("Addition str: {}", sno1.to_string());
        assert_eq!(2, 3);  // invalid to force print
    }


}
