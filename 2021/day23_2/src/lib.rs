// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

mod burrow;

use std::fs;
use std::collections::HashMap;

use burrow::AmphiType;
use burrow::BurrowState;
use burrow::PositionState;

pub fn calculate_min_energy(file_path: &String) -> u32 {
    
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Something went wrong reading the file {}",
        file_path
    ));

    let start_pos = load_data(contents);
    let end_pos = BurrowState::new_end();

    println!("Loaded start pos {}", start_pos);
    println!("End pos {}", end_pos);

    0
}


fn load_data(contents: String) -> BurrowState {

    use AmphiType::*;
    use PositionState::*;

    let mut posns: [PositionState; 27] = [Empty ; 27];

    let mut lines = contents.lines();
    lines.next();
    lines.next();

    for (row_no, line) in lines.enumerate() {

        if line.contains("######") {
            break;
        }

        let mut chars = line.chars();

        let mut cols : [char; 4] = [' '; 4];
        
        cols[0] = chars.nth(3).unwrap();
        cols[1] = chars.nth(1).unwrap();
        cols[2] = chars.nth(1).unwrap();
        cols[3] = chars.nth(1).unwrap();

        for (col_no, col) in cols.iter().enumerate() {
            
            let posn_idx = 11 + row_no + (col_no * 4);
            
            posns[posn_idx] = match col {
                'A' => Occupied(A),
                'B' => Occupied(B),
                'C' => Occupied(C),
                'D' => Occupied(D),
                _   => {
                    println!("Invalid char {} at col no {} in line {}",
                        col, col_no, line
                    );
                    Empty
                },
            }
        }

    }

    BurrowState {
        positions : posns,
    }

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn burrow_test() {
        let result = calculate_min_energy(
            &String::from("../resources/tests/day23-2-testdata.txt")
        );
        assert_eq!(result, 44169);
    }

}
