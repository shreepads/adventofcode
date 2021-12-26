// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

pub const MAX_ROT: usize = 3;   // 24 - 1

#[repr(usize)]
pub enum ROTS {
    Xpos0 = 0,
    Xpos90 = 1,
    Xpos180 = 2,
    Xpos270 = 3,
}

pub const ROT_MATS: [[ [i32; 3]; 3]; 4] = [
    [
        [ 1,  0,  0],
        [ 0,  1,  0],
        [ 0,  0,  1]
    ],
    [
        [ 1,  0,  0],
        [ 0,  0, -1],
        [ 0,  1,  0]
    ],
    [
        [ 1,  0,  0],
        [ 0, -1,  0],
        [ 0,  0, -1]
    ],
    [
        [ 1,  0,  0],
        [ 0,  0,  1],
        [ 0, -1,  0]
    ],
];

