
use std::error::Error;
use advent::common::get_data_path;
use advent::day4::constants::WORD_SEARCH_PATH;
use advent::day4;


fn main() -> Result<(), Box<dyn Error>> {
    let path = get_data_path(WORD_SEARCH_PATH).unwrap();
    match day4::count_xmas(path) {
        Ok(val) => println!("result: {}", val),
        Err(e) => panic!("{}", e),
    }
    Ok(())
}

