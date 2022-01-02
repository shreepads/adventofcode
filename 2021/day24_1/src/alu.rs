// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    Div,
    Mod,
    Eql
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Expr(Expression),
    Val(i32),
    Input(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    op: Operation,
    left: Box<Value>,
    right: Box<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alu {
    w: Value,
    x: Value,
    y: Value,
    z: Value,
    current_input: u8,
}

impl Alu {
    pub fn new() -> Alu {

        use crate::Value::Val;

        Alu {
            w: Val(0),
            x: Val(0),
            y: Val(0),
            z: Val(0),
            current_input: 0,
        }
    }

    pub fn process_instruction(&mut self, instr: String) {

        let mut parts = instr.split(' ');

        let op = parts.next().unwrap();

        let left = parts.next().unwrap();

        match op {
            "inp" => self.process_inp(left),
            "add" => {
                let right = parts.next().unwrap();
                self.process_add(left, self.right_val(right));
            },
            "mul" => {
                let right = parts.next().unwrap();
                self.process_mul(left, self.right_val(right));
            },
            "div" => {
                let right = parts.next().unwrap();
                self.process_div(left, self.right_val(right));
            },
            "mod" => {
                let right = parts.next().unwrap();
                self.process_mod(left, self.right_val(right));
            },
            "eql" => {
                let right = parts.next().unwrap();
                self.process_eql(left, self.right_val(right));
            },
            _     => {
                println!("Invalid instruction: {}", instr);
            }
        }
    }

    fn right_val(&self, right: &str) -> Value {

        use crate::Value::Val;

        // figure out if right is a register or value
        match right.parse::<i32>() {
            Ok(num) => Val(num),
            Err(_)  => match right {
                "w" => self.w.clone(),
                "x" => self.x.clone(),
                "y" => self.y.clone(),
                "z" => self.z.clone(),
                _   => {
                    println!("Invalid right: {}", right);
                    Val(0)
                },                
            },
        }

    }


    fn process_inp(&mut self, left: &str) {

        use crate::Value::Input;

        match left {
            "w" => self.w = Input(self.current_input),
            "x" => self.x = Input(self.current_input),
            "y" => self.y = Input(self.current_input),
            "z" => self.z = Input(self.current_input),
            _   => println!("Invalid inp: {}", left),
        }

        self.current_input += 1;
    }

    fn process_add(&mut self, left: &str, right_val: Value) {

        use crate::Value::*;
        use crate::Operation::Add;

        if right_val == Val(0) {return;}
            
        let left_val: Value = match left {
            "w" => self.w.clone(),
            "x" => self.x.clone(),
            "y" => self.y.clone(),
            "z" => self.z.clone(),
            _   => {
                println!("Invalid add left: {}", left);
                Val(0)
            },
        };

        // if both left and right are values add and store in left
        if let Val(right_v) = right_val {
            if let Val(left_v) = left_val {
                match left {
                    "w" => self.w = Val(right_v + left_v),
                    "x" => self.x = Val(right_v + left_v),
                    "y" => self.y = Val(right_v + left_v),
                    "z" => self.z = Val(right_v + left_v),
                    _   => {
                        println!("Invalid add left: {}", left);
                    },
                };
            }
        }

        // else form new expression and store in left


    }

    fn process_mul(&mut self, left: &str, right_val: Value) {

    }

    fn process_div(&mut self, left: &str, right_val: Value) {

    }

    fn process_mod(&mut self, left: &str, right_val: Value) {

    }

    fn process_eql(&mut self, left: &str, right_val: Value) {

    }

}