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

/* Revisit using explicit to_string method
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.pair {
            if let Some(left) = &self.left {
                let l = left.borrow();
                if let Some(right) = &self.right {
                    let r = right.borrow();
                    write!(f, "[{},{}]", l, r)
                } else {
                    write!(f, "{}", self.value.unwrap())
                }
            } else {
                write!(f, "{}", self.value.unwrap())
            }
        } else {
            write!(f, "{}", self.value.unwrap())
        }
    }
}
*/

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

    pub fn magnitude(&self) -> u32 {
        0
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_sno_pair() {
        let result = Number::new("[1,2]".to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }

    #[test]
    fn new_sno_pair_lt() {
        let result = Number::new("[[1,2],3]".to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }

    #[test]
    fn new_sno_pair_rt() {
        let result = Number::new("[9,[8,7]]".to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }

    #[test]
    fn new_sno_two_pairs() {
        let result = Number::new("[[1,9],[8,5]]".to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }
}
