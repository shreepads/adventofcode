// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    pub fn from_str(point_str: &str) -> Result<Point, String> {
        let mut num_iter = point_str.split(",");

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;

        if let Some(x_str) = num_iter.next() {
            if let Ok(xx) = x_str.parse::<i32>() {
                x = xx;
            } else {
                return Err(format!("Invalid point string {}", point_str));
            }
        } else {
            return Err(format!("Invalid point string {}", point_str));
        }

        if let Some(y_str) = num_iter.next() {
            if let Ok(yy) = y_str.parse::<i32>() {
                y = yy;
            } else {
                return Err(format!("Invalid point string {}", point_str));
            }
        } else {
            return Err(format!("Invalid point string {}", point_str));
        }

        if let Some(z_str) = num_iter.next() {
            if let Ok(zz) = z_str.parse::<i32>() {
                z = zz;
            } else {
                return Err(format!("Invalid point string {}", point_str));
            }
        } else {
            return Err(format!("Invalid point string {}", point_str));
        }

        Ok(Point { x, y, z })
    }

    pub fn euc_dist_sq(&self, point: Point) -> i64 {
        ((self.x - point.x) as i64).pow(2) + ((self.y - point.y) as i64).pow(2) + ((self.z - point.z) as i64).pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_from_str() {
        let result = Point::from_str(&String::from("86533,36374,88149"));
        assert_eq!(result, Ok(Point::new(86533, 36374, 88149)));
    }

    #[test]
    fn test_invalid_point_from_str() {
        let result = Point::from_str(&String::from("86533,abcd,88149"));
        assert_eq!(
            result,
            Err(String::from("Invalid point string 86533,abcd,88149"))
        );
    }

    #[test]
    fn test_invalid_point_from_str2() {
        let result = Point::from_str(&String::from("86533,88149"));
        assert_eq!(
            result,
            Err(String::from("Invalid point string 86533,88149"))
        );
    }

    #[test]
    fn test_point_euc_dist_sq() {
        let result = Point::new(3, 5, 7).euc_dist_sq(Point::new(1, 2, 3));
        assert_eq!(result, 29);
    }
}
