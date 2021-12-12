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

    let length = report_data.get(&0u32).unwrap().len();
    let count = report_data.len();

    println!("Calculating metric, length={}, count={}, most-common={}",
        length,
        count,
        most_common
    );

    for column in 0..length {

        if report_data.len() == 1 {
            break;
        }

        let column_total = report_data.iter().fold(
            0,
            |acc, (_, row)| acc + row[column]
        );

        let bit_criteria = calculate_bit_criteria(column_total, count, most_common);

        let mut removal_keys: Vec<u32> = Vec::new();

        for (key, row) in report_data.iter() {
            if row[column] == bit_criteria {
                removal_keys.push(*key);
            }
        }

        // Not needed as keys de-referenced above as *key
        // let clone_removal_keys: Vec<u32> = removal_keys.iter().copied().collect();

        for key in removal_keys.iter() {
            report_data.remove(key);
        }

    }

    // Retrive only record left
    let (_, last_record) = report_data.iter().next().unwrap();

    convert_metric(last_record)
}

fn calculate_bit_criteria(column_total: u32, count: usize, most_common:bool) -> u32 {

    println!("Calcluating bit criteria: column_total={}, count={}, most_common={}",
        column_total,
        count,
        most_common
    );

    1u32
}

fn convert_metric (record: &Vec<u32>) -> u32{

    println!("Last record: {:?}", record);

    3u32
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
