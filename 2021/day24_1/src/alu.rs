// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

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
    Val(i64),
    Input(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    op: Operation,
    left: Box<Value>,
    right: Box<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alu {
    pub vars: HashMap<String, Value>,
    current_input: usize,
}

impl Alu {
    pub fn new() -> Alu {

        use crate::Value::Val;

        Alu {
            vars: HashMap::from([
                ("w".to_string(), Val(0)),
                ("x".to_string(), Val(0)),
                ("y".to_string(), Val(0)),
                ("z".to_string(), Val(0)),
            ]),
            current_input: 0,
        }
    }

    pub fn process_instruction(&mut self, instr: String) {

        let mut parts = instr.split(' ');

        let op = parts.next().unwrap();

        let left = parts.next().unwrap();
        if left.len() > 1  ||  !"wxyz".contains(left) {
            println!("Invalid left in instr: {}", instr);
        }

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
        match right.parse::<i64>() {
            Ok(num) => Val(num),
            Err(_)  => self.vars.get(right).unwrap().clone(),
        }

    }


    fn process_inp(&mut self, left: &str) {

        use crate::Value::Input;

        self.vars.insert(left.to_string(), Input(self.current_input));
        self.current_input += 1;
    }

    fn process_add(&mut self, left: &str, right_val: Value) {

        use crate::Value::*;
        use crate::Operation::Add;

        if right_val == Val(0) {return;}                     // x + 0 == x
            
        let left_val: Value = self.vars.get(left).unwrap().clone();

        if left_val == Val(0) {                              // 0 + x == x
            self.vars.insert(left.to_string(), right_val);
            return;
        }

        
        if let Val(right_v) = right_val {
            
            // if both left and right are values add and store in left
            if let Val(left_v) = left_val {
                self.vars.insert(left.to_string(), Val(right_v + left_v));
                return;
            }
            
            // if left expression contains Add and Val, apply (x + a) + b = (a + x) + b = x + (a + b)
            if let Expr(left_expr) = left_val.clone() {
                if left_expr.op == Add {
                    if let Val(left_expr_left_val) = *left_expr.left {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Add,
                            left: left_expr.right,
                            right: Box::new(Val(left_expr_left_val + right_v)),
                        }));
                        return;
                    }

                    if let Val(left_expr_rt_val) = *left_expr.right {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Add,
                            left: left_expr.left,
                            right: Box::new(Val(left_expr_rt_val + right_v)),
                        }));                        
                        return;
                    }

                }
            }
        }


        if let Val(left_v) = left_val {
            
            // both left and right not val at this point
            
            // if right expression contains Add and Val, apply a + (x + b) = a + (b + x)  = x + (a + b)
            if let Expr(right_expr) = right_val.clone() {
                if right_expr.op == Add {
                    if let Val(right_expr_left_val) = *right_expr.left {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Add,
                            left: right_expr.right,
                            right: Box::new(Val(right_expr_left_val + left_v)),
                        }));
                        return;
                    }

                    if let Val(right_expr_rt_val) = *right_expr.right {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Add,
                            left: right_expr.left,
                            right: Box::new(Val(right_expr_rt_val + left_v)),
                        }));                        
                        return;
                    }

                }
            }
        }


        // else form new expression and store in left
        self.vars.insert(left.to_string(), Expr(Expression {
            op: Add,
            left: Box::new(left_val),
            right: Box::new(right_val),
        }));

    }

    fn process_mul(&mut self, left: &str, right_val: Value) {

        use crate::Value::*;
        use crate::Operation::Mul;

        if right_val == Val(1) {return;}                     // x * 1 == x
            
        let left_val: Value = self.vars.get(left).unwrap().clone();

        if left_val == Val(1) {                              // 1 * x == x
            self.vars.insert(left.to_string(), right_val);
            return;
        }

        if right_val == Val(0)  ||  left_val == Val(0) {     // 0 * x == x * 0 == 0
            self.vars.insert(left.to_string(), Val(0));
            return;
        }

        
        if let Val(right_v) = right_val {

            // if both left and right are values add and store in left
            if let Val(left_v) = left_val {
                self.vars.insert(left.to_string(), Val(right_v * left_v));
                return;
            }

            
            // if left expression contains Mul and Val, apply (x * a) * b = (a * x) * b = x * (a * b)
            if let Expr(left_expr) = left_val.clone() {
                if left_expr.op == Mul {
                    if let Val(left_expr_left_val) = *left_expr.left {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Mul,
                            left: left_expr.right,
                            right: Box::new(Val(left_expr_left_val * right_v)),
                        }));
                        return;
                    }

                    if let Val(left_expr_rt_val) = *left_expr.right {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Mul,
                            left: left_expr.left,
                            right: Box::new(Val(left_expr_rt_val * right_v)),
                        }));                        
                        return;
                    }

                }
            }


        }

        if let Val(left_v) = left_val {
            
            // both left and right not val at this point
            
            // if right expression contains Mul and Val, apply a * (x * b) = a * (b * x)  = x * (a * b)
            if let Expr(right_expr) = right_val.clone() {
                if right_expr.op == Mul {
                    if let Val(right_expr_left_val) = *right_expr.left {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Mul,
                            left: right_expr.right,
                            right: Box::new(Val(right_expr_left_val * left_v)),
                        }));
                        return;
                    }

                    if let Val(right_expr_rt_val) = *right_expr.right {
                        self.vars.insert(left.to_string(), Expr(Expression {
                            op: Mul,
                            left: right_expr.left,
                            right: Box::new(Val(right_expr_rt_val * left_v)),
                        }));                        
                        return;
                    }

                }
            }
        }


        // else form new expression and store in left
        self.vars.insert(left.to_string(), Expr(Expression {
            op: Mul,
            left: Box::new(left_val),
            right: Box::new(right_val),
        }));

    }


    fn process_div(&mut self, left: &str, right_val: Value) {

        use crate::Value::*;
        use crate::Operation::Div;

        if right_val == Val(0) {
            println!("Division by zero");
            return;
        }

        if right_val == Val(1) {        // x / 1 == x
            return;
        }
            
        let left_val: Value = self.vars.get(left).unwrap().clone();

        if left_val == Val(0) {         // 0 / x == 0
            return;
        }

        // if both left and right are values add and store in left
        if let Val(right_v) = right_val {
            if let Val(left_v) = left_val {
                self.vars.insert(left.to_string(), Val(left_v / right_v));
                return;
            }
        }

        // else form new expression and store in left
        self.vars.insert(left.to_string(), Expr(Expression {
            op: Div,
            left: Box::new(left_val),
            right: Box::new(right_val),
        }));

    }

    fn process_mod(&mut self, left: &str, right_val: Value) {

        use crate::Value::*;
        use crate::Operation::Mod;

        if right_val == Val(0) {
            println!("Mod by zero");
            return;
        }

        if right_val == Val(1) {                             // x mod 1 == 0
            self.vars.insert(left.to_string(), Val(0));
            return;
        }
            
        let left_val: Value = self.vars.get(left).unwrap().clone();

        if left_val == Val(0) {                              // 0 mod x == 0
            return;
        }

        // if both left and right are values add and store in left
        if let Val(right_v) = right_val {
            if let Val(left_v) = left_val {
                self.vars.insert(left.to_string(), Val(left_v % right_v));
                return;
            }
        }

        // else form new expression and store in left
        self.vars.insert(left.to_string(), Expr(Expression {
            op: Mod,
            left: Box::new(left_val),
            right: Box::new(right_val),
        }));

    }

    fn process_eql(&mut self, left: &str, right_val: Value) {

        use crate::Value::*;
        use crate::Operation::Eql;

        let left_val: Value = self.vars.get(left).unwrap().clone();
        
        // if both left and right are values, compare and store in left
        if let Val(right_v) = right_val {
            if let Val(left_v) = left_val {
                self.vars.insert(left.to_string(), Val(
                    if right_v == left_v {
                        1
                    } else {
                        0
                    }
                ));
                return;
            }
        }
        

        // If left and right expressions match then set left to 1, can't set 0 if not equal
        if left_val == right_val {
            self.vars.insert(left.to_string(), Val(1));
            return;
        }

        // form new expression and store in left
        self.vars.insert(left.to_string(), Expr(Expression {
            op: Eql,
            left: Box::new(left_val),
            right: Box::new(right_val),
        }));


    }

    pub fn calculate_z(&self, input: [i64; 14]) -> i64 {

        let z_val = self.vars.get("z").unwrap();

        let z_calc = Self::calc(z_val.clone(), input);

        z_calc
    }

    fn calc(val : Value, input : [i64; 14]) -> i64 {

        use crate::Value::*;
        use crate::Operation::*;

        match val {
            Val(v)           => v,
            Input(i)         => input[i],
            Expr(expr) => {
                match expr.op {
                    Add => Self::calc(*expr.left, input) + Self::calc(*expr.right, input),
                    Mul => Self::calc(*expr.left, input) * Self::calc(*expr.right, input),
                    Div => Self::calc(*expr.left, input) / Self::calc(*expr.right, input),
                    Mod => Self::calc(*expr.left, input) % Self::calc(*expr.right, input),
                    Eql => if Self::calc(*expr.left, input) == Self::calc(*expr.right, input) {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }

}