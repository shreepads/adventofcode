// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;
use std::collections::HashMap;

use crate::path::MAX_PATHS;
use crate::path::PATHS;

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub enum AmphiType {
    A,
    B,
    C,
    D,
}

impl AmphiType {
    pub fn energy(&self) -> u32 {   // because Rust can't do static HashMaps

        use self::AmphiType::*;

        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    pub fn to_char(&self) -> char {

        use self::AmphiType::*;

        match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub enum PositionState {
    Empty,
    Occupied(AmphiType),
}

impl PositionState {
    pub fn to_char(&self) -> char {
        use self::PositionState::*;

        match self {
            Empty => '☐',
            Occupied(at) => at.to_char(),
        }

    }
}

pub const MAX_POS: usize = 27;         // number of positions in the burrow

pub const NOSTOP_POS: [usize; 4] = [2, 4, 6, 8];
pub const AHOME_POS: [usize; 4] = [11, 12, 13, 14];
pub const BHOME_POS: [usize; 4] = [15, 16, 17, 18];
pub const CHOME_POS: [usize; 4] = [19, 20, 21, 22];
pub const DHOME_POS: [usize; 4] = [23, 24, 25, 26];


#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub struct BurrowState {
    pub positions: [PositionState; MAX_POS],
}

impl BurrowState {

    pub fn new_end() -> BurrowState {  
        
        use self::AmphiType::*;
        use self::PositionState::*;

        let mut posns: [PositionState; MAX_POS] = [Empty ; MAX_POS];

        for pos in AHOME_POS { posns[pos] = Occupied(A); }
        for pos in BHOME_POS { posns[pos] = Occupied(B); }
        for pos in CHOME_POS { posns[pos] = Occupied(C); }
        for pos in DHOME_POS { posns[pos] = Occupied(D); }

        BurrowState {
            positions: posns,
        }
    }


    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push('\n');
        for i in 0..11 {str.push(self.positions[i].to_char());}
        str.push('\n');
        for row_no in 0..4 {
            str.push_str(&format!("  {} {} {} {}\n",
                self.positions[11 + row_no].to_char(),
                self.positions[15 + row_no].to_char(),
                self.positions[19 + row_no].to_char(),
                self.positions[23 + row_no].to_char()
            ));
        }

        str
    }
}

impl fmt::Display for BurrowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

