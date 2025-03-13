use std::{fmt::write, fs};
use std::path::PathBuf;
use rustc_hash::{FxHashMap, FxHashSet};

pub mod constants {
    pub const INPUT_PATH: &str = "day23/input.txt";
}

type Graph = FxHashMap<String, FxHashSet<String>>;

fn graph_from_input(input: &String) -> Graph {
    let mut result = FxHashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        if result.contains_key(a) {
            result.entry(a.to_string()).and_modify(|m: &mut FxHashSet<String>| { m.insert(b.to_string()); });
        } else {
            result.entry(a.to_string()).or_default();
            result.get_mut(&a.to_string()).unwrap().insert(b.to_string());
        }
        if result.contains_key(b) {
            result.entry(b.to_string()).and_modify(|m: &mut FxHashSet<String>| { m.insert(a.to_string()); });
        } else {
            result.entry(b.to_string()).or_default();
            result.get_mut(&b.to_string()).unwrap().insert(a.to_string());
        }
    }
    result
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn find_networks(g: &Graph) -> Vec<(String, String, String)> {
    let mut visits: FxHashSet<String> = FxHashSet::default();
    let mut result = vec![];
    
    for (start_node, children) in g.iter() {
        let mut child_visits: FxHashSet<String> = FxHashSet::default();
        visits.insert(start_node.clone());
        for child_node in children.iter() {
            if visits.contains(child_node) || child_visits.contains(child_node) {
                continue;
            }
            for childs_child in g.get(child_node).unwrap() {
                if visits.contains(childs_child) || child_visits.contains(childs_child) {
                    continue;
                }
                if g.get(childs_child).unwrap().contains(start_node) {
                    result.push((start_node.clone(), child_node.clone(), childs_child.clone()));
                    child_visits.insert(child_node.clone());
                }
            }
        }
    }
    result
}

pub fn find_largest_network(g: &Graph) -> Vec<String> {
    let mut networks: Vec<Vec<String>> = find_networks(g).into_iter().map(|n| vec![n.0, n.1, n.2]).collect();
    for n in networks.iter_mut() {
        n.sort();
    }

    println!("network ({}) contenders remaining: {}", networks[0].len(), networks.len());

    loop {
        let mut next_networks = vec![];
        let mut next_dedupe: FxHashSet<Vec<String>> = FxHashSet::default();
        for i in 0..networks.len() {
            for (candidate, _children) in g.iter() {
                if !networks[i].contains(candidate) {
                    let mut fully_connected = true;
                    for net_node in networks[i].iter() {
                        if !g.get(candidate).unwrap().contains(net_node) {
                            fully_connected = false;
                        }
                    }
                    if fully_connected {
                        let mut to_add = networks[i].clone();
                        to_add.push(candidate.clone());
                        to_add.sort();
                        if !next_dedupe.contains(&to_add) {
                            next_networks.push(to_add.clone());
                            next_dedupe.insert(to_add);
                        }
                    }
                }
            }
        }
        networks = next_networks;
        println!("network ({}) contenders remaining: {}", networks[0].len(), networks.len());
        if networks.len() == 1 {
            return networks[0].clone();
        }
        if networks.len() == 0 {
            panic!("should not get here");
        }
    }
}

pub fn _solution1(input: &String) -> isize {
    let g = graph_from_input(&input);

    let networks = find_networks(&g);

    let mut result = 0;
    for network in networks.iter() {
        if network.0.starts_with('t') || network.1.starts_with('t') || network.2.starts_with('t') {
            result += 1;
        }
    }
    result
}

pub fn solution2(path: &PathBuf) -> String {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> String {
    let g = graph_from_input(&input);
    let network = find_largest_network(&g);
    network.join(",").to_string()
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn day_23_1_1() {
        let path = common::get_test_data_path("day23/case1.txt").unwrap();
        let input =  fs::read_to_string(path).unwrap();
        let g = graph_from_input(&input);
        println!("{:#?}", g);
        let networks = find_networks(&g);
        println!("{:#?}", networks);
        assert_eq!(networks.len(), 12);
    }

    #[test]
    fn day_23_1_2() {
        let path = common::get_test_data_path("day23/case1.txt").unwrap();
        assert_eq!(solution1(&path), 7);
    }

    #[test]
    fn day_23_2_1() {
        let path = common::get_test_data_path("day23/case1.txt").unwrap();
        let input =  fs::read_to_string(path).unwrap();
        let g = graph_from_input(&input);
        println!("{:#?}", g);
        let networks = find_largest_network(&g);
        println!("{:#?}", networks);
        assert_eq!(networks.len(), 4);
    }

    #[test]
    fn day_23_2_2() {
        let path = common::get_test_data_path("day23/case1.txt").unwrap();
        assert_eq!(solution2(&path), String::from("co,de,ka,ta"));
    }
}
