// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    left: Option<Box<Number>>,
    right: Option<Box<Number>>,
    value: Option<u32>,
    pair: bool,
    depth: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackElement {
    SfishNumber(Number),
    FirstOpenBracket,
    OpenBracket,
}

impl Number {
    pub fn new(line: String) -> Number {
        let mut root_s_no = Number {
            left: None,
            right: None,
            value: None,
            pair: true,    // root element always a pair
            depth: 0,
        };

        let mut stack: Vec<StackElement> = Vec::new();

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
                                    let s_no = Number {
                                        left: Some(Box::new(sno1)),
                                        right: Some(Box::new(sno2)),
                                        value: None,
                                        pair: true,
                                        depth: 0,
                                    };
                                    stack.push(StackElement::SfishNumber(s_no));
                                },
                                StackElement::FirstOpenBracket => {
                                    // map sno1 and sno2 to root_sno
                                    root_s_no.left = Some(Box::new(sno1));
                                    root_s_no.right = Some(Box::new(sno2));
                                },
                                _ => println!("Error couldn't pop open bracket after sno1")
                            }
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
                    let sno = Number {
                        left: None,
                        right: None,
                        value: Some(val),
                        pair: false,
                        depth: 0,                        
                    };
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
        println!("Sfish Number: {:?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }

    #[test]
    fn new_sno_pair_lt() {
        let result = Number::new("[[1,2],3]".to_string());
        println!("Sfish Number: {:?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }

    #[test]
    fn new_sno_pair_rt() {
        let result = Number::new("[9,[8,7]]".to_string());
        println!("Sfish Number: {:?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }

    #[test]
    fn new_sno_two_pairs() {
        let result = Number::new("[[1,9],[8,5]]".to_string());
        println!("Sfish Number: {:?}", result);
        assert_eq!(2, 4);  // invalid to force print
    }
}
