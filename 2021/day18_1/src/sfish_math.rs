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

        // TODO increase depth of all nodes

        // create new parent node
        let new_rootnode = Node {
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

    pub fn reduce(&mut self) {
        
        let mut done = false;

        while !done {
            
            let mut exploding_complete = false;

            // find leftmost node more than 5 deep and explode it
            if let Some(explode_id) = self.leftmost_explode_id() {
                // explode it
                let mut sfish_string = String::new();
                self.stringify(&mut sfish_string, explode_id);
                println!("Exploding node {}: {}", explode_id, sfish_string);

                // add part1 value to first regular number on left
                self.add_to_leftid(explode_id);
                self.add_to_rightid(explode_id);

                // replace with 0
                self.replace_with_zero(explode_id);

                //done = true;    // remove once done
            } else {
                // no more to explode
                exploding_complete = true;
            }

            // do we split if we have exploded one? TO DO

            let mut split_performed = false;

            // find node to split and split it
            if let Some(split_id) = self.get_split_id() {

                let mut sfish_string = String::new();
                self.stringify(&mut sfish_string, split_id);
                println!("Splitting node {}: {}", split_id, sfish_string);

                self.split(split_id);

                // Need to go back to explode
                split_performed = true;
            }

            // if exploding complete and no split performed, done
            if exploding_complete && !split_performed {
                done = true;
            }

        }   // while !done
    }

    fn get_split_id(&self) -> Option<usize> {

        // find node with number > 10
        for (i, node) in self.nodes.iter().enumerate() {
            
            if node.depth == 0 {
                // orphan node
                continue;
            }

            // check if it is a value node
            if node.pair {
                continue;
            }
            
            if node.value > Some(9) {
                return Some(i);
            }

        }

        None
    
    }

    fn split(&mut self, split_id: usize) {

        let split_value = self.nodes[split_id].value.unwrap();
        let split_depth = self.nodes[split_id].depth;
        let old_parent_id = self.nodes[split_id].parent_id.unwrap();

        let part1_value = split_value / 2;
        let part2_value = split_value - part1_value;

        // Create and add part1 node
        let part1 = Node {
            part1_id: None,
            part2_id: None,
            parent_id: None,
            value: Some(part1_value),
            left_id: None,
            right_id: None,
            pair: false,
            depth: split_depth + 1,
        };

        self.nodes.push(part1);
        let part1_id = self.nodes.len() - 1;

        // Create and add part2 node
        let part2 = Node {
            part1_id: None,
            part2_id: None,
            parent_id: None,
            value: Some(part2_value),
            left_id: None,
            right_id: None,
            pair: false,
            depth: split_depth + 1,
        };

        self.nodes.push(part2);
        let part2_id = self.nodes.len() - 1;

        // Create and add parent node
        let new_parent = Node {
            part1_id: Some(part1_id),
            part2_id: Some(part2_id),
            parent_id: Some(old_parent_id),
            value: None,
            left_id: None,
            right_id: None,
            pair: true,
            depth: split_depth,
        };

        self.nodes.push(new_parent);
        let new_parent_id = self.nodes.len() - 1;

        // Update parent id in parts
        self.nodes[part1_id].parent_id = Some(new_parent_id);
        self.nodes[part2_id].parent_id = Some(new_parent_id);

        // Update old parent to point to this new parent
        if self.nodes[old_parent_id].part1_id == Some(split_id) {
            self.nodes[old_parent_id].part1_id = Some(new_parent_id);
        } else if self.nodes[old_parent_id].part2_id == Some(split_id) {
            self.nodes[old_parent_id].part2_id = Some(new_parent_id);
        } else {
            println!("Old parent {} doesn't have split id {} as child", old_parent_id, split_id)
        }


        // Set split node depth to 0 so it doesn't get picked up again
        self.nodes[split_id].depth = 0;
    }

    fn leftmost_explode_id(&self) -> Option<usize> {
        
        for (i, node) in self.nodes.iter().enumerate() {
            
            //println!("Checking node {}", i);
            
            // check depth
            if node.depth < 4 {
                continue;
            }
            
            // check if it is a leaf node
            if !node.pair {
                continue;
            }

            // check if parts are leaf nodes
            if let Some(part1) = node.part1_id {
                if let Some(part2) = node.part2_id {
                    if !self.nodes[part1].pair  &&  !self.nodes[part2].pair {
                        return Some(i);
                    }
                } else {
                    println!("No part2 for node {} with pair", i);
                }
            } else {
                println!("No part1 for node {} with pair", i);
            }
        }

        None
    }

    fn add_to_leftid(&mut self, explode_id: usize) {

        let part1_id = self.nodes[explode_id].part1_id.unwrap();
        let part1_value = self.nodes[part1_id].value.unwrap();

        let mut parent_id = self.nodes[explode_id].parent_id.unwrap();
        let mut child_id = explode_id;
        let mut left_id = explode_id;

        let mut done = false;

        // traverse up till find parent whose part2 is in chain
        while !done {
            
            let parent_node = self.nodes[parent_id];
            let child_node = self.nodes[child_id];

            if parent_node.part1_id == Some(child_id) {
                // need to go further up, unless reached root
                if parent_id == self.rootnode_id {
                    // there is no left node
                    return;
                }

                child_id = parent_id;
                parent_id = parent_node.parent_id.unwrap();
            } else {
                // found node to descend
                done = true;
            }
        }

        // switch to part1 chain and traverse down till last part2
        child_id = self.nodes[parent_id].part1_id.unwrap();
        done = false;

        while !done {
            
            let child_node = self.nodes[child_id];

            // if child_id is a leaf node we're done
            if !child_node.pair {
                left_id = child_id;
                done = true;
                continue;
            }

            // traverse down part2
            parent_id = child_id;
            child_id = child_node.part2_id.unwrap();
        }

        // did we find the left Id
        if left_id != explode_id {
            // increment value

            let mut sfish_string = String::new();
            self.stringify(&mut sfish_string, left_id);
            println!("Adding {} to left node {}: {}", part1_value, left_id, sfish_string);

            if let Some(mut value) = self.nodes[left_id].value {
                self.nodes[left_id].value = Some(value + part1_value);
            }
        }

        return;
    }


    fn add_to_rightid(&mut self, explode_id: usize) {

        let part2_id = self.nodes[explode_id].part2_id.unwrap();
        let part2_value = self.nodes[part2_id].value.unwrap();

        let mut parent_id = self.nodes[explode_id].parent_id.unwrap();
        let mut child_id = explode_id;
        let mut right_id = explode_id;

        let mut done = false;

        // traverse up till find parent whose part1 is in chain
        while !done {
            
            let parent_node = self.nodes[parent_id];
            let child_node = self.nodes[child_id];

            if parent_node.part2_id == Some(child_id) {
                // need to go further up, unless reached root
                if parent_id == self.rootnode_id {
                    // there is no right node
                    return;
                }

                child_id = parent_id;
                parent_id = parent_node.parent_id.unwrap();
            } else {
                // found node to descend
                done = true;
            }
        }

        // switch to part2 chain and traverse down till last part1
        child_id = self.nodes[parent_id].part2_id.unwrap();
        done = false;

        while !done {
            
            let child_node = self.nodes[child_id];

            // if child_id is a leaf node we're done
            if !child_node.pair {
                right_id = child_id;
                done = true;
                continue;
            }

            // traverse down part1
            parent_id = child_id;
            child_id = child_node.part1_id.unwrap();
        }

        // did we find the right Id
        if right_id != explode_id {
            // increment value

            let mut sfish_string = String::new();
            self.stringify(&mut sfish_string, right_id);
            println!("Adding {} to right node {}: {}", part2_value, right_id, sfish_string);

            if let Some(mut value) = self.nodes[right_id].value {
                self.nodes[right_id].value = Some(value + part2_value);
            }
        }

        return;
    }

    
    fn replace_with_zero(&mut self, explode_id: usize) {

        let depth = self.nodes[explode_id].depth;
        let parent_id = self.nodes[explode_id].parent_id.unwrap();
        let mut parent_node = self.nodes[parent_id];

        let zero_node = Node {
            part1_id: None,
            part2_id: None,
            parent_id: Some(parent_id),
            value: Some(0),
            left_id: None,
            right_id: None,
            pair: false,
            depth: depth,
        };

        self.nodes.push(zero_node);
        
        // Set parent's child part to explode id to zero
        if parent_node.part1_id == Some(explode_id) {
            self.nodes[parent_id].part1_id = Some(self.nodes.len() - 1);
        } else {
            self.nodes[parent_id].part2_id = Some(self.nodes.len() - 1);
        }

        // Set explode node depth to 0 so it doesn't get picked up again
        self.nodes[explode_id].depth = 0;

        return

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
    fn new_sno_complex() {
        let input = "[[[2,3],[[4,5],6]],[9,[8,7]]]";
        let result = Number::new(input.to_string());
        println!("Sfish Number: {:#?}", result);
        assert_eq!(input, result.to_string());
    }

    #[test]
    fn add_1() {
        let mut sno1 = Number::new("[1,2]".to_string());
        let sno2 = Number::new("[[3,4],5]".to_string());
        sno1.add(&sno2);
        println!("Addition: {:#?}", sno1);
        assert_eq!("[[1,2],[[3,4],5]]", sno1.to_string());
    }

    #[test]
    fn add_2() {
        let mut sno1 = Number::new("[2,3]".to_string());
        let sno2 = Number::new("[[4,5],6]".to_string());
        let sno3 = Number::new("[9,[8,7]]".to_string());
        sno1.add(&sno2);
        sno1.add(&sno3);
        println!("Addition: {:#?}", sno1);
        assert_eq!("[[[2,3],[[4,5],6]],[9,[8,7]]]", sno1.to_string());
    }


    #[test]
    fn explode_none_simple() {
        let input = "[2,3]";
        let mut sno = Number::new(input.to_string());
        sno.reduce();
        println!("Reduced: {}", sno);
        assert_eq!(input, sno.to_string());
    }

    #[test]
    fn explode_none_complex() {
        let input = "[[[2,3],[[4,5],6]],[9,[8,7]]]";
        let mut sno = Number::new(input.to_string());
        sno.reduce();
        println!("Reduced: {}", sno);
        assert_eq!(input, sno.to_string());
    }


    #[test]
    fn single_explode_rt() {
        let mut sno = Number::new("[[[[[9,8],1],2],3],4]".to_string());
        sno.reduce();
        println!("Reduced: {}", sno);
        assert_eq!("[[[[0,9],2],3],4]", sno.to_string());  // force print
    }


    #[test]
    fn single_explode_lt() {
        let mut sno = Number::new("[7,[6,[5,[4,[3,2]]]]]".to_string());
        sno.reduce();
        println!("Exploded: {}", sno);
        assert_eq!("[7,[6,[5,[7,0]]]]", sno.to_string());    // force print
    }


    #[test]
    fn double_explode() {
        let mut sno = Number::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string());
        sno.reduce();
        println!("Exploded: {}", sno);
        assert_eq!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", sno.to_string());    // force print
    }

    
    #[test]
    fn explode_split_explode() {
        let mut sno = Number::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
        sno.reduce();
        println!("Reduced: {}", sno);
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", sno.to_string());    // force print
    }
    
}
