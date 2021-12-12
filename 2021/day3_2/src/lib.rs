use std::fs;
use std::collections::HashMap;

pub fn calculate_oxygen_co2(file_path: &String) -> (u32, u32) {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut report_data: HashMap<u32, Vec<u32>> = HashMap::new();

    init_report_data(&mut report_data, contents);

    // Need a copy to run both checks
    let mut report_data_2 = report_data.clone();

    println!("Data row count: {}", report_data.len());
    println!("Data 2 row count: {}", report_data_2.len());

    println!("First 3 data records:");
    println!("Record 0: {:?}", report_data.get(&0u32).unwrap());
    println!("Record 1: {:?}", report_data.get(&1u32).unwrap());
    println!("Record 2: {:?}", report_data.get(&2u32).unwrap());

    let oxygen = calculate_metric(&mut report_data, true);

    let co2 = calculate_metric(&mut report_data_2, false);

    (oxygen, co2)
}

fn init_report_data(report_data: &mut HashMap<u32, Vec<u32>>, contents: String ) {

    let lines = contents.lines();
    let mut row_no: u32 = 0;

    for line in lines {

        let mut row: Vec<u32> = Vec::new();

        for charac in line.chars() {
            match charac.to_digit(2) {
                Some(x) => row.push(x),
                None    => println!("Non binary digit in {}", line),
            }
        };

        report_data.insert(row_no, row);

        row_no += 1;

    }

}

fn calculate_metric(report_data: &mut HashMap<u32, Vec<u32>>, most_common: bool) -> u32 {

    if most_common {
        return 3;
    }
    else {
        return 5;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
