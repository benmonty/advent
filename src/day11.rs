use std::fs;
use std::path::PathBuf;
use std::fmt;

use constants::NUM_BLINKS;

pub mod constants {
    pub const INPUT_PATH: &str = "day11/input.txt";
    pub const NUM_BLINKS: usize = 25;
}

struct StoneTree {
    value: usize,
    children: Vec<StoneTree>,
}

impl fmt::Display for StoneTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let leaves = self.get_leaf_nodes();
        let repr = leaves.iter().map(|l| format!("{}", l.value)).collect::<Vec<_>>().join(" ");
        write!(f, "{}", repr)
    }
}

fn count_digits(num: usize) -> usize {
    let s = format!("{}", num);
    s.len()
}

fn split_in_half(num: usize) -> (usize, usize) {
    let s = format!("{}", num);
    assert!(s.len() % 2 == 0);
    let (num1, num2) = s.split_at(s.len()/2);
    (num1.parse::<usize>().unwrap(), num2.parse::<usize>().unwrap())
}

impl StoneTree {
    pub fn from(input: &String) -> Self {
        let mut children = Vec::new();
        for part in input.trim().split(' ') {
            let val = part.parse::<usize>().unwrap();
            children.push(Self { value: val, children: Vec::new() });
        }
        Self {
            value:  usize::MAX,
            children,
        }
    }

    pub fn _get_leaf_nodes<'a>(& self, t: &'a StoneTree, leaves: &mut Vec<&'a StoneTree>) -> () {
        if t.children.len() == 0 {
            leaves.push(&t)
        } else {
            for child in t.children.iter() {
                self._get_leaf_nodes(child, leaves);
            }
        }

    }

    pub fn get_leaf_nodes(&self) -> Vec<&StoneTree> {
        let mut leaves = Vec::new();

        self._get_leaf_nodes(&self, &mut leaves);
        
        leaves
    }

    pub fn step(&mut self) -> () {
        if self.children.len() == 0 {
            self.compute_children();
        } else {
            for child in self.children.iter_mut() {
                child.step();
            }
        }
    }

    pub fn compute_children(&mut self) -> () {
        let val = self.value;

        if val == 0 {
            self.children.push(Self { value: 1, children: Vec::new() })
        } else if count_digits(val) % 2 == 0 {
            let (child_val1, child_val2) = split_in_half(val);
            self.children.push(StoneTree {
                value: child_val1,
                children: vec![],
            });
            self.children.push(StoneTree {
                value: child_val2,
                children: vec![],
            });
        } else {
            let (v, is_overflow) = val.overflowing_mul(2024);
            assert!(!is_overflow); 
            self.children.push(StoneTree {
                value: v,
                children: vec![],
            });
        }
    }

    pub fn count_leaf_nodes(&self) -> usize {
        if self.children.len() == 0 {
            if self.value == usize::MAX {
                0
            } else {
                1
            }
        } else {
            let mut result = 0;
            for child in self.children.iter() {
                result += child.count_leaf_nodes();
            }
            result
        }
    }
}

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    let mut tree = StoneTree::from(input);

    for _blink_count in 0..NUM_BLINKS {
        tree.step();
    }

    tree.count_leaf_nodes()
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(_input: &String) -> usize {
    0
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example_day11_1() {
        let path = common::get_test_data_path("day11/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_example_day11_2() {
        assert!(false, "todo")
    }
}
