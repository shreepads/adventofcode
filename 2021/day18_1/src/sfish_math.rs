// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    left: Option<Rc<RefCell<Number>>>,
    right: Option<Rc<RefCell<Number>>>,
    parent: Option<Rc<RefCell<Number>>>,
    value: Option<u32>,
    pair: bool,
    depth: u32,
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum StackElement {
    SfishNumber(Rc<RefCell<Number>>),
    FirstOpenBracket,
    OpenBracket,
}

impl Number {
    pub fn new(line: String) -> Number {
        let mut root_s_no = Number {
            left: None,
            right: None,
            parent: None,
            value: None,
            pair: true,    // root element always a pair
            depth: 1,
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
                    // pop Numbers and open bracket
                    if let StackElement::SfishNumber(sno2) = stack.pop().unwrap() {
                        if let StackElement::SfishNumber(sno1) = stack.pop().unwrap() {
                            // check if first open bracket or not
                            match stack.pop().unwrap() {
                                StackElement::OpenBracket => {
                                    // create new sno and push
                                    let s_no = Rc::new(RefCell::new(Number {
                                        left: Some(Rc::clone(&sno1)),
                                        right: Some(Rc::clone(&sno2)),
                                        parent: None,
                                        value: None,
                                        pair: true,
                                        depth: depth,
                                    }));
                                    sno1.borrow_mut().parent = Some(Rc::clone(&s_no));
                                    sno2.borrow_mut().parent = Some(Rc::clone(&s_no));
                                    stack.push(StackElement::SfishNumber(s_no));
                                },
                                StackElement::FirstOpenBracket => {
                                    // map sno1 and sno2 to root_sno
                                    root_s_no.left = Some(Rc::clone(&sno1));
                                    root_s_no.right = Some(Rc::clone(&sno2));
                                },
                                _ => println!("Error couldn't pop open bracket after sno1")
                            };
                            // reduce depth
                            depth -= 1;
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
                    let sno = Rc::new(RefCell::new(Number {
                        left: None,
                        right: None,
                        parent: None,
                        value: Some(val),
                        pair: false,
                        depth: depth,                        
                    }));
                    stack.push(StackElement::SfishNumber(sno));
                },
            }
        }
                    
        root_s_no
    }

    pub fn magnitude(&self) -> u32 {
        self.value.unwrap()
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
