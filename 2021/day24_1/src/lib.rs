// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod alu;
mod alu2;

use std::fs;

use alu::*;
use alu2::*;

pub fn calculate_max_serialno(file_path: &String) -> i64 {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    'outer: for i1 in (1..=9).rev() {
        for i2 in (1..=9).rev() {
            for i3 in (1..=9).rev() {
                for i4 in (1..=9).rev() {
                    for i5 in (1..=9).rev() {
                        for i6 in (1..=9).rev() {
                            for i7 in (1..=9).rev() {
                                for i8 in (1..=9).rev() {
                                    for i9 in (1..=9).rev() {
                                        for i10 in (1..=9).rev() {
                                            for i11 in (1..=9).rev() {
                                                for i12 in (1..=9).rev() {
                                                    for i13 in (1..=9).rev() {
                                                        for i14 in (1..=9).rev() {
                                                            let mut alu = Alu2::new();
                                                            let input: [i64; 14] = [
                                                                i1, i2, i3, i4, i5, i6, i7, i8, i9,
                                                                i10, i11, i12, i13, i14,
                                                            ];

                                                            for line in contents.lines() {
                                                                alu.process_instruction(
                                                                    line.to_string(),
                                                                    input,
                                                                );
                                                            }

                                                            if *alu.vars.get("z").unwrap() == 0 {
                                                                println!(
                                                                    "Found serial number: {:?}",
                                                                    input
                                                                );
                                                                break 'outer;
                                                            } else {
                                                                //println!("Not serial number: {:?}", input);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tiny_add() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata1.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn tiny_add_mul() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata2.txt"));
        assert_eq!(result, 5);
    }

    #[test]
    fn tiny_add_mul_div() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata3.txt"));
        assert_eq!(result, 4);
    }

    #[test]
    fn tiny_add_mul_div_mod() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata4.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn tiny_add_commutative() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata5.txt"));
        assert_eq!(result, 2);
    }

    #[test]
    fn tiny_mul_commutative() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata6.txt"));
        assert_eq!(result, 2);
    }

    #[test]
    fn tiny_eql() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata7.txt"));
        assert_eq!(result, 1);
    }

    #[test]
    fn calc_z() {
        let result =
            calculate_max_serialno(&String::from("../resources/tests/day24-1-testdata8.txt"));
        assert_eq!(result, 1);
    }
}
