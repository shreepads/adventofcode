use std::fs;

#[derive(Debug, Clone)]
pub struct Position {
    pub horizontal: i64,
    pub depth:      i64,
}

pub fn calculate_position(start_posn: Position, file_path: &String) -> Position {
    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let commands = contents.lines();

    let mut final_posn = start_posn.clone();

    for command in commands {
        final_posn = translate_position(final_posn, command);
    }

    final_posn

}

fn translate_position(posn: Position, command: &str) -> Position {
    
    println!("Executing command:{}", command);

    posn
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
