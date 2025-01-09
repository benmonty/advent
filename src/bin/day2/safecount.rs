use std::error::Error;
use advent::common::get_data_path;
use advent::day2::constants::REPORT_FILE_PATH;
use advent::day2;

fn main() -> Result<(), Box<dyn Error>> {
    let path = get_data_path(REPORT_FILE_PATH).unwrap();
    match day2::count_safe_reports(path) {
        Ok(result) => println!("{}", result),
        Err(e) => panic!("{:?}", e),
    };
    Ok(())
}
