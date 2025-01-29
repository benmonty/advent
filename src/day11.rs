use std::fs;
use std::path::PathBuf;
use std::fmt;
use std::collections::VecDeque;
use rustc_hash::FxHashMap;

use constants::{NUM_BLINKS_PT1, NUM_BLINKS_PT2};

pub mod constants {
    pub const INPUT_PATH: &str = "day11/input.txt";
    pub const NUM_BLINKS_PT1: usize = 25;
    pub const NUM_BLINKS_PT2: usize = 75;
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

    pub fn _get_leaf_count(&self, mut depth_vals: VecDeque<(usize, usize)>, num_steps: usize, cache: &FxHashMap<(usize, usize), usize>) -> usize {
        let mut leaf_count = 0;

        while depth_vals.len() != 0 {
            let processing = depth_vals.pop_front().unwrap();
            let mut current_step = processing.0;
            let processing_val = processing.1;
            let mut depth_distance = num_steps - current_step;
            if let Some(v) = cache.get(&(depth_distance, processing_val)) {
                leaf_count += v;
            } else {
                let vals = self.compute_child_values(processing_val);
                current_step += 1;
                let mut pushes = 0;
                depth_distance = num_steps - current_step;
                if let Some(val2) = vals.1 {
                    if let Some(v) = cache.get(&(depth_distance, val2)) {
                        leaf_count += v;
                    } else {
                        depth_vals.push_front((current_step, val2));
                        pushes += 1;
                    }
                }
                if let Some(v) = cache.get(&(depth_distance, vals.0)) {
                    leaf_count += v
                } else {
                    depth_vals.push_front((current_step, vals.0));
                    pushes += 1;
                }
                if current_step == num_steps {
                    for _i in 0..pushes {
                        depth_vals.pop_front();
                    }
                    leaf_count += pushes;
                }
            }
        }
        leaf_count
    }

    pub fn get_leaf_count(&self, depth: usize) -> usize {
        let leaf_vals: VecDeque<(usize, usize)> = self.get_leaf_nodes().iter().map(|t| (0, t.value)).collect();

        let mut cache: FxHashMap<(usize, usize), usize> = FxHashMap::default();

        for factor in 0..=8 {
            for val in 0..100000 {
               for num_steps in (factor*10 + 1)..(factor*10 + 10) {
                    let to_cache = VecDeque::from([(0, val)]);
                    let leaf_count_in_n_steps = self._get_leaf_count(to_cache, num_steps, &cache);
                    cache.entry((num_steps, val)).or_insert(leaf_count_in_n_steps);
                    println!("[{}] iter {} complete", factor, num_steps);
               }
            }
        }


        self._get_leaf_count(leaf_vals, depth, &cache)
    }

    pub fn step(&mut self) -> () {
        if self.children.len() == 0 {
            self.build_children();
        } else {
            for child in self.children.iter_mut() {
                child.step();
            }
        }
    }

    pub fn compute_child_values(&self, val: usize) -> (usize, Option<usize>) {
        if val == 0 {
            (1, None)
        } else if count_digits(val) % 2 == 0 {
            let (child_val1, child_val2) = split_in_half(val);
            (child_val1, Some(child_val2))
        } else {
            let (v, is_overflow) = val.overflowing_mul(2024);
            assert!(!is_overflow); 
            (v, None)
        }

    }

    pub fn build_children(&mut self) -> () {
        let child_vals = self.compute_child_values(self.value);
        self.children.push(Self {
            value: child_vals.0,
            children: Vec::new(),
        });
        match child_vals.1 {
            Some(v) => {
                self.children.push(Self {
                    value: v,
                    children: Vec::new()
                });

            },
            None => (),
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
    let tree = StoneTree::from(input);

    tree.get_leaf_count(NUM_BLINKS_PT1)

    //for _blink_count in 0..NUM_BLINKS_PT1 {
    //    tree.step();
    //}
    //
    //tree.count_leaf_nodes()
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> usize {
    let tree = StoneTree::from(input);

    tree.get_leaf_count(NUM_BLINKS_PT2)
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
}
