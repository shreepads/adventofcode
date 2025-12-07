// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fs;

pub fn sum_invalid_ids(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut sum = 0;

    for id_range in file_contents.split(",") {
        let mut id_iter = id_range.split("-");

        let start_id = id_iter.next().unwrap().parse::<u64>().unwrap();
        let end_id = id_iter.next().unwrap().parse::<u64>().unwrap();

        let start_id_len = num_digits(start_id);
        let end_id_len = num_digits(end_id);

        // Skip range if numbers don't have even number of digits
        if start_id_len == end_id_len {
            if start_id_len % 2 != 0 {
                continue;
            }
        }

        sum += (start_id..end_id + 1)
            .filter(|x| num_digits(*x) % 2 == 0)
            .filter(|x| !is_valid_id(*x, 2))
            .fold(0, |acc, x| acc + x);
    }

    sum
}

pub fn sum_invalid_ids2(file_path: &String) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut sum = 0;

    for id_range in file_contents.split(",") {
        let mut id_iter = id_range.split("-");

        let start_id = id_iter.next().unwrap().parse::<u64>().unwrap();
        let end_id = id_iter.next().unwrap().parse::<u64>().unwrap();

        for id in start_id..end_id + 1 {
            if id < 10 {
                continue;
            };

            for repeat_num in list_factors_except_one(num_digits(id)) {
                if !is_valid_id(id, repeat_num) {
                    sum += id;
                    break;
                }
            }
        }
    }

    sum
}

// Why didn't I think of this first
fn is_valid_id(id: u64, repeat_num: u64) -> bool {
    let id_len = num_digits(id);
    let mut id_mut = id;

    // Expect only factors of id_str_len to be used here
    assert!(id_len % repeat_num == 0);

    let chunk_length = (id_len / repeat_num) as u32;

    let mut nums = vec![];

    for _ in 0..repeat_num {
        nums.push(id_mut % 10_u64.pow(chunk_length));
        id_mut = id_mut / 10_u64.pow(chunk_length);
    }

    // heck if all nums same
    if nums.windows(2).all(|w| w[0] == w[1]) {
        return false;
    }

    true
}

// Find the number of digits in a number faster than using to_string.len()
#[inline(always)]
fn num_digits(num: u64) -> u64 {
    if num < 10 {
        return 1;
    };
    if num < 100 {
        return 2;
    };
    if num < 1000 {
        return 3;
    };
    if num < 10000 {
        return 4;
    };
    if num < 100000 {
        return 5;
    };
    if num < 1000000 {
        return 6;
    };
    if num < 10000000 {
        return 7;
    };
    if num < 100000000 {
        return 8;
    };
    if num < 1000000000 {
        return 9;
    };
    if num < 10000000000 {
        return 10;
    };

    num.to_string().len() as u64
}

// List all factors of num excluding 1 but including num itself
// As per problem checks num is 2 digits at least
// Use simple method as number of digits is < 10
fn list_factors_except_one(num: u64) -> Vec<u64> {
    let mut factor_list = vec![];

    assert!(num > 1);

    factor_list.push(num);

    // If num is one of the smaller primes return
    if vec![2, 3, 5, 7, 11, 13, 17, 19, 23].contains(&num) {
        return factor_list;
    };

    // Hard code for some smaller non-primes
    if num == 4 {
        return vec![2, 4];
    };
    if num == 6 {
        return vec![2, 3, 6];
    };
    if num == 8 {
        return vec![2, 4, 8];
    };
    if num == 9 {
        return vec![3, 9];
    };
    if num == 10 {
        return vec![2, 5, 10];
    };

    for i in 2..num {
        if num % i == 0 {
            factor_list.push(i)
        };
    }

    factor_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_invalid_ids() {
        let result = sum_invalid_ids(&String::from("../resources/test-input/day02-test.txt"));
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_sum_invalid_ids2() {
        let result = sum_invalid_ids2(&String::from("../resources/test-input/day02-test.txt"));
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_is_valid_id1() {
        let result = is_valid_id(10, 2);
        assert!(result);
    }

    #[test]
    fn test_is_valid_id2() {
        let result = is_valid_id(44, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_valid_id3() {
        let result = is_valid_id(4443, 2);
        assert!(result);
    }

    #[test]
    fn test_is_valid_id4() {
        let result = is_valid_id(1188511885, 2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_list_factors_except_one1() {
        let result = list_factors_except_one(2);
        assert_eq!(result, vec!(2));
    }

    #[test]
    fn test_list_factors_except_one2() {
        let result = list_factors_except_one(3);
        assert_eq!(result, vec!(3));
    }

    #[test]
    fn test_list_factors_except_one3() {
        let result = list_factors_except_one(4);
        assert_eq!(result, vec!(2, 4));
    }

    #[test]
    fn test_list_factors_except_one4() {
        let result = list_factors_except_one(6);
        assert_eq!(result, vec!(2, 3, 6));
    }

    #[test]
    fn test_list_factors_except_one5() {
        let result = list_factors_except_one(30);
        assert_eq!(result, vec!(30, 2, 3, 5, 6, 10, 15));
    }
}
