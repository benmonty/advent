use std::error::Error;
use advent::common;
use advent::day14;
use advent::day14::constants::INPUT_PATH;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(INPUT_PATH).unwrap();
    println!("{}", day14::solution1(&path));
    Ok(())
}