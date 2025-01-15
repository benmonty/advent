use std::error::Error;
use advent::common;
use advent::day19;
use advent::day19::constants::INPUT_PATH;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(INPUT_PATH).unwrap();
    println!("{}", day19::solution1(&path));
    Ok(())
}
