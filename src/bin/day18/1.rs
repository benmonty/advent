use std::error::Error;
use advent::common;
use advent::day18;
use advent::day18::constants::INPUT_PATH;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(INPUT_PATH).unwrap();
    println!("{}", day18::solution1(&path));
    Ok(())
}
