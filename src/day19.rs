use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::thread;

use rustc_hash::{FxHashSet, FxHashMap};



pub mod constants {
    pub const INPUT_PATH: &str = "day19/input.txt";
}

fn parse_towels(input: &String) -> FxHashSet<String> {
    let towels_line = input.lines().next().unwrap();
    towels_line.split(", ").map(|i| i.to_string()).collect()
}

fn parse_designs(input: &String) -> Vec<String> {
    let mut lines = input.lines();
    lines.next(); // towel patterns
    lines.next(); // blank

    lines.map(|s| s.to_string()).collect()
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

fn can_build(towels: &FxHashSet<String>, design: &str, mut max_len: usize) -> bool {
    if design == "" {
        return true;
    }

    max_len = design.len().min(max_len);

    //println!("total design: {}", design);

    for i in (1..=max_len).rev() {
        //println!("\tchecking len: {}", i);
        //println!("\tdesign subset: {}", &design[0..i]);
        if towels.contains(&design[design.len()-i..design.len()]) {
            //println!("\t\tCONTAINS");
            //println!("\t\tremaining: {}", &design[i..]);
            if can_build(&towels, &design[0..design.len()-i], max_len) {
                return true;
            }
        }
    }
    false
}

fn chunkiest_solution<'a>(towels: &'a FxHashSet<String>, design: &'a str, mut max_len: usize) -> Option<Vec<&'a str>> {
    if design == "" {
        return Some(vec![]);
    }

    max_len = design.len().min(max_len);

    //println!("total design: {}", design);

    for i in (1..=max_len).rev() {
        //println!("\tchecking len: {}", i);
        //println!("\tdesign subset: {}", &design[0..i]);
        let next_design = &design[design.len()-i..design.len()];
        if towels.contains(next_design) {
            //println!("\t\tCONTAINS");
            //println!("\t\tremaining: {}", &design[i..]);
            match chunkiest_solution(&towels, &next_design, max_len) {
                Some(mut sol) => {
                    sol.push(next_design);
                    return Some(sol);
                },
                None => (),
            }
        }
    }
    None
}

fn count_combos<'a>(towels: &FxHashSet<String>, design: &'a str, mut max_len: usize) -> isize {
    if design == "" {
        return 1;
    }

    let mut count = 0;

    max_len = design.len().min(max_len);

    for i in (1..=max_len).rev() {
        if towels.contains(&design[design.len()-i..design.len()]) {
            let next_des = &design[0..design.len()-i];
            let _can_build: bool;
            _can_build = can_build(&towels, next_des, max_len);
            if _can_build {
                count += count_combos(&towels, next_des, max_len);
            }
        }
    }

    count
}

pub fn _solution1(input: &String) -> isize {
    let towels = parse_towels(&input);
    //println!("{:#?}", towels);
    let designs = parse_designs(&input);

    let mut max_len = 0;

    for towel in towels.iter() {
        max_len = max_len.max(towel.len());
    }


    let mut num_possible = 0;
    //let mut prefixes = VecDeque::new();

    for d in designs.iter() {
        println!("processing: {}", d);
        if can_build(&towels, d, max_len) {
            num_possible += 1;
        }
    }

    num_possible
}

pub fn solution2(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn get_all_combinations(a: &Vec<String>, b: &Vec<String>) -> Vec<(String, String)> {
    let mut result = vec![];
    for a_elt in a.iter() {
        for b_elt in b.iter() {
            result.push((a_elt.clone(), b_elt.clone()));
            result.push((b_elt.clone(), a_elt.clone()));
        }
    }
    result
}


pub fn _solution2(input: &String) -> isize {
    let towels = parse_towels(&input);
    println!("towels count: {}", towels.len());
    let designs = parse_designs(&input);

    let mut max_towel_len = 0;
    for t in towels.iter() {
        if t.len() > max_towel_len {
            max_towel_len = t.len();
        }
    }

    max_towel_len = max_towel_len.min(input.len());
    let mut sum = 0;
    for d in designs.iter() {
        println!("processing: {}", d);
        sum += count_design_combos(&towels, d, max_towel_len);
    }
    sum
}

pub fn count_design_combos(towels: &FxHashSet<String>, design: &str, biggest_towel_len: usize) -> isize {
    let midpt = design.len()/2;
    let mut gap_crossing_patterns: Vec<(String, usize)> = vec![];
    let max_towel_len = biggest_towel_len.min(midpt + 1);

    for i in 1..max_towel_len {
        let base_idx = midpt - i;
        let min_len = midpt - base_idx + 1;
        for t in towels.iter() {
            if t.len() >= min_len {
                if towels.contains(&design[base_idx..base_idx + t.len()]) && &design[base_idx..base_idx + t.len()] == t {
                    gap_crossing_patterns.push((t.clone(), base_idx));
                }
            }
        }
    }
    let mut sum = 0;

    if max_towel_len <= biggest_towel_len {
        sum += count_combos(&towels, &design[0..midpt], max_towel_len)
               * count_combos(&towels, &design[midpt..], max_towel_len);

        for p in gap_crossing_patterns.iter() {
            sum += count_combos(&towels, &design[0..p.1], max_towel_len)
                   * count_combos(&towels, &design[p.1 + p.0.len()..], max_towel_len);
        }
    } else {
        sum += count_design_combos(&towels, &design[0..midpt], biggest_towel_len)
               * count_design_combos(&towels, &design[midpt..], biggest_towel_len);

        for p in gap_crossing_patterns.iter() {
            sum += count_design_combos(&towels, &design[0..p.1], biggest_towel_len)
                   * count_design_combos(&towels, &design[p.1 + p.0.len()..], biggest_towel_len);
        }
    }

    sum
}

//pub fn _solution2(input: &String) -> isize {
//    //println!("{:#?}", towels);
//    let i = input.clone();
//    let towels = parse_towels(&i);
//    let designs = parse_designs(&input);
//
//    let mut max_len = 0;
//
//    for towel in towels.iter() {
//        max_len = max_len.max(towel.len());
//    }
//
//    let mut combos = 0;
//    for d in cloned_cd.iter() {
//        let mut build_cache = FxHashMap::default();
//        for i in 0..d.len()-1 {
//            let next_des = &d[0..d.len()-i];
//            build_cache.entry(next_des).or_insert(can_build(&t, next_des, max_len));
//        }
//
//        println!("processing: {}", d);
//        if can_build(&t, d, max_len) {
//            combos += count_combos(&t, d, max_len, &mut build_cache);
//        }
//    }
//    combos
//}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common::{self, get_test_data_path};

    #[test]
    fn day_19_1() {
        let path = get_test_data_path("day19/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 6, "correct possible combos identified");
    }

    #[test]
    fn day_19_2() {
        let path = get_test_data_path("day19/case1.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 16, "correct possible combos identified");
    }
}
