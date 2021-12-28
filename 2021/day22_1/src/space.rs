// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct Cuboid {
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
}

impl Cuboid {
    pub fn new(line: String, max_limit: i32) -> Option<Cuboid> {
        // parse string
        let mut coords = line.split(",y=");

        let mut coordsx = coords.next().unwrap().split("=");
        coordsx.next();
        let x_str = coordsx.next().unwrap();
        let mut x_coords = x_str.split("..");
        let x1 = x_coords.next().unwrap().parse::<i32>().unwrap();
        let x2 = x_coords.next().unwrap().parse::<i32>().unwrap();

        let yz_str = coords.next().unwrap();
        let mut rem_coords = yz_str.split(",z=");

        let y_str = rem_coords.next().unwrap();
        let mut y_coords = y_str.split("..");
        let y1 = y_coords.next().unwrap().parse::<i32>().unwrap();
        let y2 = y_coords.next().unwrap().parse::<i32>().unwrap();

        let z_str = rem_coords.next().unwrap();
        let mut z_coords = z_str.split("..");
        let z1 = z_coords.next().unwrap().parse::<i32>().unwrap();
        let z2 = z_coords.next().unwrap().parse::<i32>().unwrap();

        if  (-max_limit..=max_limit).contains(&x1) && 
            (-max_limit..=max_limit).contains(&x2) &&
            (-max_limit..=max_limit).contains(&y1) &&
            (-max_limit..=max_limit).contains(&y2) &&
            (-max_limit..=max_limit).contains(&z1) &&
            (-max_limit..=max_limit).contains(&y2)
        {
            return Some(Cuboid {
                x1: x1, x2: x2,
                y1: y1, y2: y2,
                z1: z1, z2: z2, 
            });
        } else {
            return None;
        }
    }

    pub fn to_string(&self) -> String {
        format!("x={}..{},y={}..{},z={}..{}", self.x1, self.x2, self.y1, self.y2, self.z1, self.z2)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_cuboid() {
        let input = "on x=-16..35,y=-41..10,z=-47..6";
        let result = Cuboid::new(input.to_string(), 50);
        assert_eq!(result.unwrap().to_string(), "x=-16..35,y=-41..10,z=-47..6");
    }

    #[test]
    fn maxlimit_cuboid_neg() {
        let input = "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877";
        let result = Cuboid::new(input.to_string(), 50);
        assert_eq!(result, None);
    }


    #[test]
    fn maxlimit_cuboid_pos() {
        let input = "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877";
        let result = Cuboid::new(input.to_string(), 100000);
        assert_eq!(result.unwrap().to_string(), "x=-54112..-39298,y=-85059..-49293,z=-27449..7877");
    }

}