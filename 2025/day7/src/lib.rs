// Copyright (c) 2025 Shreepad Shukla
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Space {
    Empty,
    Start,
    Splitter,
}

pub fn beam_splits(file_path: &String) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let diagram = load_diagram(file_contents);

    let mut splitters_reached: HashSet<(usize, usize)> = HashSet::new();

    for (i, diag_line) in diagram.iter().enumerate() {
        // No splitters on first line
        if i == 0 {
            continue;
        }

        // Iterate through spaces on the line
        'jloop: for (j, space) in diag_line.iter().enumerate() {
            if *space == Space::Splitter {
                // Determine height to check from
                // Either top of diagram or splitter or start
                let mut check_height = i - 1;

                for height in (0..i).rev() {
                    // If empty go up
                    if diagram[height][j] == Space::Empty {
                        continue;
                    }

                    // If splitter then stop and set check height
                    if diagram[height][j] == Space::Splitter {
                        check_height = height;
                        break;
                    }

                    // If Start then add to splitters reached and break out
                    if diagram[check_height][j] == Space::Start {
                        splitters_reached.insert((i, j));
                        break 'jloop;
                    }
                }

                // Look for used splitter to left from check_height to i-1
                if j > 0 {
                    for height in (check_height..i).rev() {
                        if splitters_reached.contains(&(height, j - 1)) {
                            // Add to splitters reached and break out
                            splitters_reached.insert((i, j));
                            continue 'jloop;
                        }
                    }
                }

                // Look for used splitter to right from check_height to i-1
                if j < diag_line.len() - 1 {
                    for height in (check_height..i).rev() {
                        if splitters_reached.contains(&(height, j + 1)) {
                            // Add to splitters reached and break out
                            splitters_reached.insert((i, j));
                            continue 'jloop;
                        }
                    }
                }
            }
        }
    }

    splitters_reached.len()
}

fn load_diagram(file_contents: String) -> Vec<Vec<Space>> {
    let mut diagram = vec![];

    for (i, line) in file_contents.lines().enumerate() {
        // Skip alternate empty lines
        if i % 2 == 1 {
            continue;
        }

        let mut diag_line = vec![];

        for c in line.chars() {
            match c {
                '.' => diag_line.push(Space::Empty),
                'S' => diag_line.push(Space::Start),
                '^' => diag_line.push(Space::Splitter),
                _ => panic!("Invalid char in file"),
            }
        }

        diagram.push(diag_line);
    }

    assert_eq!(diagram[0].len(), diagram[diagram.len() - 1].len());

    diagram
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_splits() {
        let result = beam_splits(&String::from("../resources/test-input/day07-test.txt"));
        assert_eq!(result, 21);
    }

    #[test]
    fn test_load_diagram() {
        let result = load_diagram(String::from(
            r#"...S...
.......
...^...
.......
..^.^..
.......
.^.^.^.
......."#,
        ));
        assert_eq!(
            result,
            vec!(
                vec!(
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                    Space::Start,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty
                ),
                vec!(
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                    Space::Splitter,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty
                ),
                vec!(
                    Space::Empty,
                    Space::Empty,
                    Space::Splitter,
                    Space::Empty,
                    Space::Splitter,
                    Space::Empty,
                    Space::Empty
                ),
                vec!(
                    Space::Empty,
                    Space::Splitter,
                    Space::Empty,
                    Space::Splitter,
                    Space::Empty,
                    Space::Splitter,
                    Space::Empty
                ),
            )
        );
    }
}
