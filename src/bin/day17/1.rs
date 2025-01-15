use std::error::Error;
use advent::common;
use advent::day17;
use advent::day17::constants::INPUT_PATH;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(INPUT_PATH).unwrap();
    println!("{}", day17::solution1(&path));
    Ok(())
}
