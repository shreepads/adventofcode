// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

//use crate::burrow::MAX_POS;
pub const MAX_POS: usize = 3;

// needed for fixed array const
pub const MAX_PATHS: usize = 1;    // max number of paths, from a door node
pub const MAX_STEPS: usize = 14;   // max number of steps in a path, from A to D homes

pub enum PathStep {    
    Nil,
    Pos(usize),
}

use self::PathStep::*;

pub const PATHS : [[[PathStep; MAX_STEPS] ; MAX_PATHS] ; MAX_POS] = [
    [ // paths from position 0
        [Pos(1), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil]
    ],
    [ // paths from position 1
        [Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil]
    ],
    [ // paths from position 2
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil]
    ],
];

