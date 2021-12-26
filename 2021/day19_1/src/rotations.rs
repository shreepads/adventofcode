// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub const MAX_ROT: usize = 7;   // 24 - 1

#[repr(usize)]
pub enum ROTS {
    Xpos0   = 0,
    Xpos90  = 1,
    Xpos180 = 2,
    Xpos270 = 3,
    Xneg0   = 4,
    Xneg90  = 5,
    Xneg180 = 6,
    Xneg270 = 7,
}

pub const ROT_MATS: [[ [i32; 3]; 3]; 8] = [
    [  // Xpos0
        [ 1,  0,  0],
        [ 0,  1,  0],
        [ 0,  0,  1]
    ],
    [  // Xpos90
        [ 1,  0,  0],
        [ 0,  0, -1],
        [ 0,  1,  0]
    ],
    [  // Xpos180
        [ 1,  0,  0],
        [ 0, -1,  0],
        [ 0,  0, -1]
    ],
    [ // Xpos270
        [ 1,  0,  0],
        [ 0,  0,  1],
        [ 0, -1,  0]
    ],
    [  // Xneg0
        [-1,  0,  0],
        [ 0,  1,  0],
        [ 0,  0,  1]
    ],
    [  // Xneg90
        [-1,  0,  0],
        [ 0,  0, -1],
        [ 0,  1,  0]
    ],
    [  // Xneg180
        [-1,  0,  0],
        [ 0, -1,  0],
        [ 0,  0, -1]
    ],
    [ // Xneg270
        [-1,  0,  0],
        [ 0,  0,  1],
        [ 0, -1,  0]
    ],
];

