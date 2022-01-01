// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use crate::burrow::MAX_POS;

// needed for fixed array const
pub const MAX_PATHS: usize = 5;    // 5, max number of paths, from a door node
pub const MAX_STEPS: usize = 14;   // max number of steps in a path, from A to D homes

pub enum PathStep {    
    Nil,
    Pos(usize),
}

use self::PathStep::*;

pub const PATHS : [[[PathStep; MAX_STEPS] ; MAX_PATHS] ; MAX_POS] = [
    [ // paths from position 0
        [Pos(1), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(1), Pos(2), Pos(3), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(1), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil],
        [Pos(1), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 1
        [Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 2 - no stop posn
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 3
        [Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 4 - no stop posn
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 5
        [Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 6 - no stop posn
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 7
        [Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 8 - no stop posn
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 9
        [Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil],
        [Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil],
        [Pos(8), Pos(7), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 10
        [Pos(9), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil],
        [Pos(9), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil],
        [Pos(9), Pos(8), Pos(7), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(9), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 11
        [Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil],
        [Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil],
    ],
    [ // paths from position 12
        [Pos(11), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil],
        [Pos(11), Pos(2), Pos(3), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil],
        [Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil],
    ],
    [ // paths from position 13
        [Pos(12), Pos(11), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil],
        [Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil],
        [Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil],
        [Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil],
    ],
    [ // paths from position 14
        [Pos(13), Pos(12), Pos(11), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(13), Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil],
        [Pos(13), Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil],
        [Pos(13), Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil],
        [Pos(13), Pos(12), Pos(11), Pos(2), Pos(3), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26)],
    ],
    [ // paths from position 15
        [Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 16
        [Pos(15), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(15), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(15), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(15), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(15), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 17
        [Pos(16), Pos(15), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(16), Pos(15), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil],
        [Pos(16), Pos(15), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil],
        [Pos(16), Pos(15), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil],
        [Pos(16), Pos(15), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil],
    ],
    [ // paths from position 18
        [Pos(17), Pos(16), Pos(15), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(17), Pos(16), Pos(15), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil],
        [Pos(17), Pos(16), Pos(15), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil],
        [Pos(17), Pos(16), Pos(15), Pos(4), Pos(5), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil],
        [Pos(17), Pos(16), Pos(15), Pos(4), Pos(5), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil],
    ],
    [ // paths from position 19
        [Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 20
        [Pos(19), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(19), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(19), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil, Nil],
        [Pos(19), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(19), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 21
        [Pos(20), Pos(19), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil],
        [Pos(20), Pos(19), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(20), Pos(19), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil],
        [Pos(20), Pos(19), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil],
        [Pos(20), Pos(19), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 22
        [Pos(21), Pos(20), Pos(19), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil],
        [Pos(21), Pos(20), Pos(19), Pos(6), Pos(7), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(21), Pos(20), Pos(19), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil],
        [Pos(21), Pos(20), Pos(19), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil],
        [Pos(21), Pos(20), Pos(19), Pos(6), Pos(7), Pos(8), Pos(23), Pos(24), Pos(25), Pos(26), Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 23
        [Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil, Nil],
        [Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil, Nil],
        [Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil, Nil],
        [Pos(8), Pos(7), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 24
        [Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil, Nil],
        [Pos(23), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil, Nil],
        [Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil, Nil],
        [Pos(23), Pos(8), Pos(7), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 25
        [Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil, Nil],
        [Pos(24), Pos(23), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14), Nil],
        [Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil, Nil],
        [Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil, Nil],
    ],
    [ // paths from position 26
        [Pos(25), Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(1), Pos(0), Nil, Nil],
        [Pos(25), Pos(24), Pos(23), Pos(8), Pos(9), Pos(10), Nil, Nil, Nil, Nil, Nil, Nil, Nil, Nil],
        [Pos(25), Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(3), Pos(2), Pos(11), Pos(12), Pos(13), Pos(14)],
        [Pos(25), Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(5), Pos(4), Pos(15), Pos(16), Pos(17), Pos(18), Nil, Nil],
        [Pos(25), Pos(24), Pos(23), Pos(8), Pos(7), Pos(6), Pos(19), Pos(20), Pos(21), Pos(22), Nil, Nil, Nil, Nil],
    ],

];



