use std::error::Error;
use std::path::Path;
use advent::day1::result_from_file;
use advent::day1::constants::LOCATIONS_FILE_PATH;


fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(LOCATIONS_FILE_PATH);
    match result_from_file(path) {
        Ok(result) => println!("{}", result),
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
