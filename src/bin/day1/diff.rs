use std::error::Error;
use advent::day1::result_from_file;
use advent::day1::constants::LOCATIONS_FILE_PATH;
use advent::common::get_data_path;


fn main() -> Result<(), Box<dyn Error>> {
    let path = get_data_path(LOCATIONS_FILE_PATH).unwrap();
    match result_from_file(&path) {
        Ok(result) => println!("{}", result),
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
