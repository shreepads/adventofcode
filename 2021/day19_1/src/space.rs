// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;

use crate::rotations::MAX_ROT;
use crate::rotations::ROT_MATS;
use crate::rotations::ROTS;

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Point {

    pub fn new_zero() -> Point {
        Point {x:0, y:0, z:0}
    }

    pub fn new(line: String) -> Point {
        
        // parse string
        let mut coords = line.split(",");

        let x = coords.next().unwrap().parse::<i32>().unwrap();
        let y = coords.next().unwrap().parse::<i32>().unwrap();
        let z = coords.next().unwrap().parse::<i32>().unwrap();
        
        Point {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn rotate_vec(&self) -> Vec<Point> {
        let mut rot_points: Vec<Point>  = Vec::new();

        for rot in 0..=MAX_ROT {
            rot_points.push(self.rotate_new(rot));
        }

        rot_points
    }
    
    pub fn rotate_new(&self, rotation: usize) -> Point {
        let mut new_point = Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };

        new_point.rotate(rotation);

        new_point
    } 

    pub fn rotate(&mut self, rotation: usize) {
        if rotation > MAX_ROT {
            println!("Rotation # too high: {}", rotation);
        }

        // rotate using matrix multiplication with ROT_MATS
        let rot_mat = ROT_MATS[rotation];

        // calculate rot x
        let rot_mat_rowx = rot_mat[0];
        let x = (self.x * rot_mat_rowx[0]) + (self.y * rot_mat_rowx[1]) + (self.z * rot_mat_rowx[2]);

        // calculate rot y
        let rot_mat_rowy = rot_mat[1];
        let y = (self.x * rot_mat_rowy[0]) + (self.y * rot_mat_rowy[1]) + (self.z * rot_mat_rowy[2]);

        // calculate rot z
        let rot_mat_rowz = rot_mat[2];
        let z = (self.x * rot_mat_rowz[0]) + (self.y * rot_mat_rowz[1]) + (self.z * rot_mat_rowz[2]);

        self.x = x;
        self.y = y;
        self.z = z;
    } 


    pub fn to_string(&self) -> String {
        
        format!("{},{},{}", self.x, self.y, self.z)

    }


    pub fn translate_to(&self, refpoint: Point) -> Translation {
        // Generate translation from this point to given reference point
        Translation {
            deltax: refpoint.x - self.x,
            deltay: refpoint.y - self.y,
            deltaz: refpoint.z - self.z,
        }

    }

    pub fn translate_new(&self, trans: Translation) -> Point {

        let mut new_point = Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };

        new_point.translate(trans);

        new_point

    }


    pub fn translate(&mut self, trans: Translation) {
        self.x = self.x + trans.deltax;
        self.y = self.y + trans.deltay;
        self.z = self.z + trans.deltaz;
    }

}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct Translation {
    deltax: i32,
    deltay: i32,
    deltaz: i32,
}

impl fmt::Display for Translation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Δx: {}, Δy: {}, Δz: {}", self.deltax, self.deltay, self.deltaz)
    }
}

impl Translation {
    pub fn new_zero() -> Translation {
        Translation {deltax:0, deltay:0, deltaz:0}
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_point() {
        let input = "-537,-823,-458";
        let result = Point::new(input.to_string());
        println!("Point: {}", result);
        assert_eq!(input, result.to_string());
    }

    #[test]
    fn rotate_Xpos0() {
        let input = "-537,-823,-458";
        let mut result = Point::new(input.to_string());
        result.rotate(ROTS::Xpos0 as usize);
        println!("Point: {}", result);
        assert_eq!(input, result.to_string());
    }


    #[test]
    fn rotate_Xpos90() {
        let input = "-537,-823,-458";
        let mut result = Point::new(input.to_string());
        println!("Point             : {}", result);
        result.rotate(ROTS::Xpos90 as usize);
        println!("Rotated point by {}: {}", ROTS::Xpos90 as usize, result);
        assert_eq!("-537,458,-823", result.to_string());
    }


    #[test]
    fn rotate_Xpos180() {
        let input = "-537,83,-458";
        let mut result = Point::new(input.to_string());
        println!("Point             : {}", result);
        result.rotate(ROTS::Xpos180 as usize);
        println!("Rotated point by {}: {}", ROTS::Xpos180 as usize, result);
        assert_eq!("-537,-83,458", result.to_string());
    }


    #[test]
    fn rotate_Xpos270() {
        let input = "-537,83,-458";
        let mut result = Point::new(input.to_string());
        println!("Point             : {}", result);
        result.rotate(ROTS::Xpos270 as usize);
        println!("Rotated point by {}: {}", ROTS::Xpos270 as usize, result);
        assert_eq!("-537,-458,-83", result.to_string());
    }

    #[test]
    fn rotations() {
        let input = "-537,83,-458";
        let mut result = Point::new(input.to_string());
        println!("Point             : {}", result);
        let points = result.rotate_vec();
        println!("Rotated points: {:?}", points);
        assert_eq!(MAX_ROT+1, points.len());
    }

    #[test]
    fn translation() {
        let point1 = Point::new("8,5,4".to_string());
        let point0 = Point::new("2,3,1".to_string());
        let trans = point1.translate_to(point0);
        let result = point1.translate_new(trans);

        println!("Transalating point1 {} to point0 {} using translation {}",
            point1, point0, trans
        );

        assert_eq!(point0, result);
    }


}


