use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::fmt;
use std::path::PathBuf;

use regex::Regex;

pub mod constants {
    pub const LOCATIONS_FILE_PATH: &str = "day1/locations.txt";
}

#[derive(Debug, PartialEq)]
enum LocError {
    PathError,
}

impl Error for LocError {}

impl fmt::Display for LocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "loc error")
    }
}

fn _read_columns(file_path: &PathBuf) -> Result<(Vec<u64>, Vec<u64>), Box<dyn Error>> {
    let mut col1: Vec<u64> = Vec::new();
    let mut col2: Vec<u64> = Vec::new();

    let rx = Regex::new(r"\s+").expect("invalid regex");

    for line in read_to_string(file_path).unwrap().lines() {
        let locations: Vec<&str> = rx.split(line).collect();

        let loc1 = locations[0].parse::<u64>().unwrap();
        let loc2 = locations[1].parse::<u64>().unwrap();

        col1.push(loc1);
        col2.push(loc2);
    }
    Ok((col1, col2))
}

fn _result_from_file(file_path: &PathBuf) -> Result<u64, Box<dyn Error>> {
    let (mut col1, mut col2) = _read_columns(file_path)?;

    col1.sort();
    col2.sort();

    let it = col1.iter().zip(col2.iter());
    let mut result: u64 = 0;

    for (loc1, loc2) in it {
        result += loc1.abs_diff(*loc2);
    }
    Ok(result)
}

pub fn result_from_file(file_path: &PathBuf) -> Result<u64, Box<dyn Error>> {
    match file_path.is_file() {
        true => _result_from_file(file_path),
        false => Err(Box::new(LocError::PathError)),
    }
}

fn _compute_similarity_score(file_path: &PathBuf) -> Result<u64, Box<dyn Error>> {
    let (col1, col2) = _read_columns(file_path)?;
    let mut col2_counts: HashMap<u64, u64> = HashMap::new();

    for loc in col2 {
        *col2_counts.entry(loc).or_insert(0) += 1;
    }

    let mut result: u64 = 0;
    for loc in col1 {
        let multiplier = *col2_counts.entry(loc).or_insert(0);
        result += loc * multiplier;
    }

    Ok(result)
}

pub fn compute_similarity_score(file_path: &PathBuf) -> Result<u64, Box<dyn Error>> {
    match file_path.is_file() {
        true => _compute_similarity_score(file_path),
        false => Err(Box::new(LocError::PathError)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::get_test_data_path;

    #[test]
    fn compute_result_from_file() {
        let path = get_test_data_path("day1/case1.txt").unwrap();
        let result = result_from_file(&path).unwrap();
        assert_eq!(result, 116, "computed the correct result");
    }

    #[test]
    fn similarity_score() {
        let path = get_test_data_path("day1/case2.txt").unwrap();
        let result = compute_similarity_score(&path).unwrap();
        assert_eq!(result, 31, "computed the correct result");
    }
}
