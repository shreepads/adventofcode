use std::fs;

pub fn calculate_oxygen_co2(file_path: &String) -> (i64, i64) {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let  lines = contents.lines();

    println!("Lines count: {}", lines.count());

    (3, 4)
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
