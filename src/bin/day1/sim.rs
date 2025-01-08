use std::error::Error;
use advent::day1::compute_similarity_score;
use advent::day1::constants::LOCATIONS_FILE_PATH;
use advent::common::get_data_path;


fn main() -> Result<(), Box<dyn Error>> {
    let path = get_data_path(LOCATIONS_FILE_PATH).unwrap();
    match compute_similarity_score(&path) {
        Ok(result) => println!("{}", result),
        Err(e) => panic!("{:?}", e),
    }
    Ok(())
}
