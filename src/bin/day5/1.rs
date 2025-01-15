use std::error::Error;
use advent::common;
use advent::day5;
use advent::day5::constants::PRINTER_UPDATES;

fn main() -> Result<(), Box<dyn Error>> {
    let path = common::get_data_path(PRINTER_UPDATES).unwrap();
    println!("{}", day5::compute_part1_solution(&path));
    //let raw_details =  fs::read_to_string(path).unwrap();
    //let all_details = PrintInstructions::from(&raw_details);
    ////summary.print();
    //let mut match_counts: HashMap<bool, usize> = HashMap::new();
    //for update in all_details.updates.iter() {
    //    let filtered_details = all_details.filter_for_update(update);
    //    let pairs = update.get_pairs();
    //    for p in pairs {
    //        let entry = match_counts.entry(filtered_details.rules.contains(p)).or_insert(0);
    //        *entry += 1;
    //    }
    //}
    //println!("TRUE: {}", match_counts.get(&true).unwrap());
    //println!("FALSE: {}", match_counts.get(&false).unwrap_or(&0));
        
    Ok(())
}
