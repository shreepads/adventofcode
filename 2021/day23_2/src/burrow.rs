// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::fmt;
use std::collections::HashSet;

use crate::path::PATHS;
use crate::path::PathStep::{Nil, Pos};

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub enum AmphiType {
    A,
    B,
    C,
    D,
}

impl AmphiType {
    pub fn energy(&self) -> usize {   // because Rust can't do static HashMaps

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
pub const HALLWAY_POS: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];


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

    pub fn next_states(&self) -> Vec<(usize, BurrowState)> {

        let mut next_states = Vec::new();

        for (pos, pstate) in self.positions.iter().enumerate() {
            
            if NOSTOP_POS.contains(&pos) {  // can't move from no stop locations
                continue;
            }

            match pstate {
                PositionState::Empty           => {},  // can't move from empty positions, should handle NOSTOP
                PositionState::Occupied(atype) => next_states.append(&mut self.next_states_pos(pos, *atype)),
            }
        }

        next_states
    }

    fn next_states_pos(&self, start_posn: usize, atype: AmphiType) -> Vec<(usize, BurrowState)> {

        use self::PositionState::*;
        use self::AmphiType::*;

        let mut next_states : HashSet<(usize, BurrowState)> = HashSet::new();
        let mut prev_posn = usize::MAX;

        for path in PATHS[start_posn].iter() {
            for (step_no, step) in path.iter().enumerate() {
                match step {
                    Nil       => break,  // can't continue down this path
                    Pos(new_posn) => {

                        // TODO : if entering self home, don't stop till reach last empty posn

                        match self.positions[*new_posn] {
                            Occupied(_) => break, // can't continue down this path
                            Empty       => {
                                
                                // can't stop at no stop posns
                                if NOSTOP_POS.contains(&new_posn) {
                                    prev_posn = *new_posn;
                                    continue;
                                } 

                                // if started in hallway, can't stop in hallway
                                if HALLWAY_POS.contains(&start_posn)  &&  HALLWAY_POS.contains(&new_posn) {
                                    prev_posn = *new_posn;
                                    continue;
                                }

                                // if started in a home, can't stop in same home
                                if AHOME_POS.contains(&start_posn)  &&  AHOME_POS.contains(&new_posn) {
                                    prev_posn = *new_posn;
                                    continue;
                                }

                                if BHOME_POS.contains(&start_posn)  &&  BHOME_POS.contains(&new_posn) {
                                    prev_posn = *new_posn;
                                    continue;
                                }

                                if CHOME_POS.contains(&start_posn)  &&  CHOME_POS.contains(&new_posn) {
                                    prev_posn = *new_posn;
                                    continue;
                                }

                                if DHOME_POS.contains(&start_posn)  &&  DHOME_POS.contains(&new_posn) {
                                    prev_posn = *new_posn;
                                    continue;
                                }
                                
                                
                                // break if entering another home
                                if atype == A  &&  NOSTOP_POS.contains(&prev_posn) && 
                                    (BHOME_POS.contains(&new_posn) ||  CHOME_POS.contains(&new_posn) || DHOME_POS.contains(&new_posn)) {
                                        break;
                                }

                                if atype == B  &&  NOSTOP_POS.contains(&prev_posn) && 
                                    (AHOME_POS.contains(&new_posn) ||  CHOME_POS.contains(&new_posn) || DHOME_POS.contains(&new_posn)) {
                                        break;
                                }

                                if atype == C  &&  NOSTOP_POS.contains(&prev_posn) && 
                                    (AHOME_POS.contains(&new_posn) ||  BHOME_POS.contains(&new_posn) || DHOME_POS.contains(&new_posn)) {
                                        break;
                                }

                                if atype == D  &&  NOSTOP_POS.contains(&prev_posn) && 
                                    (AHOME_POS.contains(&new_posn) ||  BHOME_POS.contains(&new_posn) || CHOME_POS.contains(&new_posn)) {
                                        break;
                                }

                                // break if entering own home but its occupied by others
                                if atype == A  &&  NOSTOP_POS.contains(&prev_posn) && AHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {break;}
                                }

                                if atype == B  &&  NOSTOP_POS.contains(&prev_posn) && BHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {break;}
                                }

                                if atype == C  &&  NOSTOP_POS.contains(&prev_posn) && CHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {break;}
                                }

                                if atype == D  &&  NOSTOP_POS.contains(&prev_posn) && DHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {break;}
                                }


                                // if entered own home, don't stop till reaching last empty posn
                                if atype == A  &&  !AHOME_POS.contains(&start_posn) && AHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {
                                        println!("Error: entered own home when others present, state: {}", self);
                                        break;
                                    }
                                    if AHOME_POS.contains(&(*new_posn + 1)) {
                                        if self.positions[*new_posn + 1] == Empty {
                                            prev_posn = *new_posn;
                                            continue;
                                        }
                                    }
                                }

                                if atype == B  &&  !BHOME_POS.contains(&start_posn) && BHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {
                                        println!("Error: entered own home when others present, state: {}", self);
                                        break;
                                    }
                                    if BHOME_POS.contains(&(*new_posn + 1)) {
                                        if self.positions[*new_posn + 1] == Empty {
                                            prev_posn = *new_posn;
                                            continue;
                                        }
                                    }
                                }

                                if atype == C  &&  !CHOME_POS.contains(&start_posn) && CHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {
                                        println!("Error: entered own home when others present, state: {}", self);
                                        break;
                                    }
                                    if CHOME_POS.contains(&(*new_posn + 1)) {
                                        if self.positions[*new_posn + 1] == Empty {
                                            prev_posn = *new_posn;
                                            continue;
                                        }
                                    }
                                }

                                if atype == D  &&  !DHOME_POS.contains(&start_posn) && DHOME_POS.contains(&new_posn) {
                                    if self.home_contains_others(atype) {
                                        println!("Error: entered own home when others present, state: {}", self);
                                        break;
                                    }
                                    if DHOME_POS.contains(&(*new_posn + 1)) {
                                        if self.positions[*new_posn + 1] == Empty {
                                            prev_posn = *new_posn;
                                            continue;
                                        }
                                    }
                                }

                                // add new state at current non-empty posn
                                let mut next_state: BurrowState = *self;
                                next_state.positions[start_posn] = Empty;
                                next_state.positions[*new_posn] = Occupied(atype);

                                let energy = (step_no + 1) * atype.energy();

                                next_states.insert((energy, next_state));
                            },
                        }

                        prev_posn = *new_posn;
                    }
                }
            }
        }

        Vec::from_iter(next_states)
    }

    fn home_contains_others(&self, atype: AmphiType) -> bool {
        
        use self::PositionState::*;
        use self::AmphiType::*;
        
        let home = match atype {
            A => AHOME_POS,
            B => BHOME_POS,
            C => CHOME_POS,
            D => DHOME_POS,
        };

        for posn in home {
           match self.positions[posn] {
               Empty           => continue,
               Occupied(home_atype) => {
                    if home_atype != atype {
                        // home contains a non atype amph
                        return true;
                    }
               },
           } 
        }

        false
    }
}


impl fmt::Display for BurrowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

