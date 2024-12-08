// Copyright (c) 2022 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;
use std::fs;

const TREE_GRID_SIDE: usize = 99;

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: u32,
    visible: bool,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vis = if self.visible { "v" } else { "i" };
        write!(f, "{}{}", self.height, vis)
    }
}

pub fn visible_trees(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut tree_grid = load_tree_grid(&file_contents);

    //print_tree(&tree_grid);

    // scan tree lines L to R

    for (row_id, tree_line) in tree_grid.iter().enumerate() {

        // skip top and bottom row
        if row_id == 0 ||  row_id == TREE_GRID_SIDE - 1 {
            break;
        }

        for (col_id, tree) in tree_line.iter().enumerate() {
            
            // skip left and right ends
            if col_id == 0 ||  col_id == TREE_GRID_SIDE - 1 {
                break;
            }
        }
    } 

    0
}

fn load_tree_grid(file_contents: &String) -> [[Tree; TREE_GRID_SIDE]; TREE_GRID_SIDE] {
    let mut tree_grid = [[Tree {
        height: 100,
        visible: false,
    }; TREE_GRID_SIDE]; TREE_GRID_SIDE];

    for (row_id, tree_line) in file_contents.lines().enumerate() {
        for (col_id, tree_ch) in tree_line.chars().enumerate() {
            let height = tree_ch.to_digit(10).unwrap();

            let visible = if row_id == 0
                || row_id == TREE_GRID_SIDE - 1
                || col_id == 0
                || col_id == TREE_GRID_SIDE - 1
            {
                true
            } else {
                false
            };

            let tree = Tree { height, visible };

            tree_grid[row_id][col_id] = tree;
        }
    }

    tree_grid
}

fn print_tree(tree_grid: &[[Tree; TREE_GRID_SIDE]; TREE_GRID_SIDE]) {
    for tree_line in tree_grid.iter() {
        for tree in tree_line.iter() {
            print!("{},", tree);
        }

        println!();
    }
}
