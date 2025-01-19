#![allow(unused_variables)]
use std::fs;
use std::path::PathBuf;

pub mod constants {
    pub const INPUT_PATH: &str = "";
}

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    0
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> usize {
    0
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example_day1() {
        assert!(false, "todo")
    }

    #[test]
    fn test_example_day2() {
        assert!(false, "todo")
    }
}
