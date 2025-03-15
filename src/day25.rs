use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Lock {
    pin_heights: Vec<isize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    tooth_heights: Vec<isize>,
}

enum ParseStage {
    Lock,
    Key,
    Nothing,
}

pub mod constants {
    pub const INPUT_PATH: &str = "day25/input.txt";
}

fn combine_heights(line: &str, heights: &mut Vec<isize>) {
    for (i, c) in line.chars().enumerate() {
        match c {
            '#' => heights[i] += 1,
            '.' => (),
            _ => panic!("unexpected input found"),
        }
    }
}

fn parse(input: &String) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = vec![];
    let mut locks = vec![];

    let mut current_key: Option<Key> = None;
    let mut current_lock: Option<Lock> = None;

    let mut parse_stage = ParseStage::Nothing;
    for l in input.lines() {
        match parse_stage {
            ParseStage::Key => {
                if l == "" {
                    parse_stage = ParseStage::Nothing;
                    keys.push(current_key.clone().unwrap().clone());
                    current_key = None;
                } else {
                    combine_heights(l, &mut current_key.as_mut().unwrap().tooth_heights);
                }
            },
            ParseStage::Lock => {
                if l == "" {
                    parse_stage = ParseStage::Nothing;
                    locks.push(current_lock.clone().unwrap().clone());
                    current_lock = None;
                } else {
                    combine_heights(l, &mut current_lock.as_mut().unwrap().pin_heights);
                }
            },
            ParseStage::Nothing => {
                if l.starts_with('#') {
                    parse_stage = ParseStage::Lock;
                    current_lock = Some(Lock { pin_heights: vec![0; l.len()] });
                } else {
                    parse_stage = ParseStage::Key;
                    current_key = Some(Key { tooth_heights: vec![-1; l.len()] });
                }
            },
        }
    }
    match (&mut current_key, &mut current_lock) {
        (Some(_key), _) => {
            keys.push(current_key.clone().unwrap().clone());
        },
        (_, Some(_lock)) => {
            locks.push(current_lock.clone().unwrap().clone());
        },
        _ => panic!("should be processing a key or lock at eof"),
    }

    (keys, locks)
}

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    let (keys, locks) = parse(&input);
    let mut fit_count = 0;
    let key_height = 7;

    for key in keys.iter() {
        for lock in locks.iter() {
            let mut fits = true;
            for t in 0..key.tooth_heights.len() {
                if key.tooth_heights[t] + lock.pin_heights[t] <= key_height - 2 {
                } else {
                    fits = false;
                }
            }
            if fits {
                fit_count += 1;
            }
        }
    }

    fit_count
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
    fn day25_1_1() {
        let path = common::get_test_data_path("day25/case1.txt").unwrap();
        assert_eq!(solution1(&path), 3);
    }

    #[test]
    fn day25_1_parse() {
        let path = common::get_test_data_path("day25/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let (keys, locks) = parse(&input);
        assert_eq!(keys, vec![
            Key { tooth_heights: vec![5, 0, 2, 1, 3]},
            Key { tooth_heights: vec![4, 3, 4, 0, 2]},
            Key { tooth_heights: vec![3, 0, 2, 0, 1]},
        ]);
        assert_eq!(locks, vec![
            Lock { pin_heights: vec![0, 5, 3, 4 , 3] },
            Lock { pin_heights: vec![1, 2, 0, 5 , 3] },
        ]);
    }

    #[test]
    fn day25_2_1() {
        assert!(false, "todo")
    }
}
