use std::fs;
use std::collections::HashSet;

pub fn calculate_oxygen_co2(file_path: &String) -> (i64, i64) {

    println!("Loading data from file:{}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut report_data: HashSet<Vec<u32>> = HashSet::new();

    init_report_data(&mut report_data, contents);

    // Need a copy to run both checks
    let mut report_data_2 = report_data.clone();

    println!("Data row count: {}", report_data.len());

    println!("Data 2 row count: {}", report_data_2.len());

    let oxygen = calculate_metric(&mut report_data, true);

    let co2 = calculate_metric(&mut report_data_2, false);

    (oxygen, co2)
}

fn init_report_data(report_data: &mut HashSet<Vec<u32>>, contents: String ) {

    let lines = contents.lines();

    for line in lines {

        let mut row: Vec<u32> = Vec::new();

        for charac in line.chars() {
            match charac.to_digit(2) {
                Some(x) => row.push(x),
                None    => println!("Non binary digit in {}", line),
            }
        };
    
        if !report_data.insert(row) {
            println!("Duplicate record!!");
        }

    }

}

fn calculate_metric(report_data: &mut HashSet<Vec<u32>>, most_common: bool) -> i64{
    if most_common {
        return 12;
    }
    else {
        return 3;
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
