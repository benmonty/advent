use std::error::Error;
use advent::common;
use advent::day5;
use advent::day5::constants::PRINTER_UPDATES;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(PRINTER_UPDATES).unwrap();
    println!("{}", day5::compute_part1_solution(&path));
    Ok(())
}
