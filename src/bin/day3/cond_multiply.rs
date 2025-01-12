use std::error::Error;
use advent::common::get_data_path;
use advent::day3::constants::MEMORY_FILE_PATH;
use advent::day3;


fn main() -> Result<(), Box<dyn Error>> {
    let path = get_data_path(MEMORY_FILE_PATH).unwrap();
    match day3::cond_multiply(path) {
        Ok(val) => println!("result: {}", val),
        Err(e) => panic!("{}", e),
    }
    Ok(())
}

