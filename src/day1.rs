use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::fmt;
use std::path::Path;

use regex::Regex;

pub mod constants {
    pub const LOCATIONS_FILE_PATH: &str = "/home/titus/code/advent/src/locations.txt";
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

fn _read_columns(file_path: &Path) -> Result<(Vec<u64>, Vec<u64>), Box<dyn Error>> {
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

fn _result_from_file(file_path: &Path) -> Result<u64, Box<dyn Error>> {
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

pub fn result_from_file(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    match file_path.is_file() {
        true => _result_from_file(file_path),
        false => Err(Box::new(LocError::PathError)),
    }
}

fn _compute_similarity_score(file_path: &Path) -> Result<u64, Box<dyn Error>> {
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

pub fn compute_similarity_score(file_path: &Path) -> Result<u64, Box<dyn Error>> {
    match file_path.is_file() {
        true => _compute_similarity_score(file_path),
        false => Err(Box::new(LocError::PathError)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn compute_result_not_a_file() {
        let path = Path::new("/asdf/foo/bar.txt");
        let result = result_from_file(path);

        let e = result.expect_err("should return a file error");
        assert_eq!(e.downcast::<LocError>().unwrap(), Box::new(LocError::PathError));
    }

    #[test]
    fn compute_result_from_file() {
        let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        pb.push("tests/data/day1/case1.txt");
        let path = Path::new(&pb);
        let result = result_from_file(path).unwrap();
        assert_eq!(result, 116, "computed the correct result");
    }

    #[test]
    fn sim_score_not_a_file() {
        let path = Path::new("/asdf/foo/bar.txt");
        let result = compute_similarity_score(path);

        let e = result.expect_err("should return a file error");
        assert_eq!(e.downcast::<LocError>().unwrap(), Box::new(LocError::PathError));
    }

    #[test]
    fn similarity_score() {
        let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        pb.push("tests/data/day1/case2.txt");
        let path = Path::new(&pb);
        let result = compute_similarity_score(path).unwrap();
        assert_eq!(result, 31, "computed the correct result");
    }
}
