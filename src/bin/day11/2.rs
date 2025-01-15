use std::error::Error;
use advent::common;
use advent::day11;
use advent::day11::constants::INPUT_PATH;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(INPUT_PATH).unwrap();
    println!("{}", day11::solution2(&path));
    Ok(())
}
