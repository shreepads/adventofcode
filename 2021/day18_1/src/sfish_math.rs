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

impl Number {
    pub fn new(line: String) -> Number {
        Number {
            left: None,
            right: None,
            value: Some(4),
            pair: false,
            depth: 0,
        }
    }

    pub fn magnitude(&self) -> u32 {
        self.value.unwrap()
    }
}