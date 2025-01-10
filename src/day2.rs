use std::path::PathBuf;
use std::error::Error;
use std::fs;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;

pub mod constants {
    pub const REPORT_FILE_PATH: &str = "day2/reports.txt";
}

struct Report {
    levels: Vec<u64>,
}

enum LevelOrdering {
    AllIncrementing,
    AllDecrementing,
    NotOrdered,
}

fn get_ordering(levels: &Vec<u64>) -> LevelOrdering {
    if levels.len() == 1 {
        LevelOrdering::AllIncrementing
    } else {
        let first_level = levels[0];
        let second_level = levels[1];
        let expected_ordering = if first_level > second_level {
            LevelOrdering::AllDecrementing
        } else if second_level > first_level {
            LevelOrdering::AllIncrementing
        } else {
            LevelOrdering::NotOrdered
        };

        let mut prev_level = first_level;

        for level in &levels[1..] {
            match expected_ordering {
                LevelOrdering::AllIncrementing => {
                    if !(*level > prev_level) {
                        return LevelOrdering::NotOrdered;
                    }
                },
                LevelOrdering::AllDecrementing => {
                    if !(*level < prev_level) {
                        return LevelOrdering::NotOrdered;
                    }
                },
                LevelOrdering::NotOrdered => return LevelOrdering::NotOrdered,
            };
            prev_level = *level;
        }
        expected_ordering
    }
}

fn gapping_valid(levels: &Vec<u64>) -> bool {
    if levels.len() == 1 {
        true
    } else {
        let mut prev_level = levels[0];
        let mut result = true;

        for level in &levels[1..] {
            let abs_diff = level.abs_diff(prev_level);
            if abs_diff > 3 || abs_diff < 1 {
                result = false;
            }
            prev_level = *level;
        }
        result
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum EdgeKind {
    Ascending,
    Descending,
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            EdgeKind::Ascending => "ASC",
            EdgeKind::Descending => "DESC",
        };
        write!(f, "{}", txt)
    }
}

struct Edge {
    cost: usize,
    dest_node: Rc<RefCell<Node>>,
    kind: EdgeKind,
}

struct Node {
    value: Option<u64>,
    edges: Vec<Rc<RefCell<Edge>>>,
}

struct LevelsGraph {
    nodes: Vec<Rc<RefCell<Node>>>,
    root: Rc<RefCell<Node>>,
}

impl fmt::Display for LevelsGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for src_node in self.nodes.iter().map(|n| n.borrow()) {
            let src_node_identifier = match src_node.value {
                Some(val) => val.to_string(),
                None => String::from("ROOT"),
            };

            write!(f, "SRC NODE: {}\n", src_node_identifier)?;

            for edge in src_node.edges.iter().map(|e| e.borrow()) {
                write!(
                    f,
                    "\tEDGE:\tto:{}\tkind:{}\tcost:{}\n",
                    edge.dest_node.borrow().value.unwrap(),
                    edge.kind,
                    edge.cost,
                )?;
            }

        }
        Ok(())
    } 
}

impl LevelsGraph {
    const MAX_LEVEL_DIFF: u64 = 3;

    fn new() -> Self {
        // initialize a root node so that we can skip
        // over the first level if needed
        let root = Rc::new(RefCell::new(Node {
            value: None,
            edges: Vec::new(),
        }));
        let nodes = Vec::from([root.clone()]);
        let graph = LevelsGraph {
            nodes: nodes,
            root: root.clone(),
        };
        graph 
    }

    fn is_safe(&self) -> bool {
        let mut all_paths = self.get_all_paths();

        all_paths.sort_by(|a, b| b.len().cmp(&a.len()));

        if all_paths[0].len() > self.nodes.len() - 2 {
            true
        } else {
            false
        }
    }

    fn _get_paths(&self, node: Rc<RefCell<Node>>, kind: EdgeKind, budget: usize) -> Vec<Vec<Rc<RefCell<Node>>>> {
        // if current node has no edges of `kind`, we're done, return empty vector
        let borrowed_node = node.borrow();
        let safe_edges = borrowed_node.edges.iter().map(|e| e.borrow()).filter(|e| e.kind == kind && e.cost <= budget);
        if safe_edges.clone().count() == 0 {
            let mut result = Vec::new();
            result.push(Vec::from([node.clone()]));
            return result;
        }

        let mut result = Vec::new();

        for edge in safe_edges {
            let new_budget = budget - edge.cost;
            let mut safe_paths = self._get_paths(edge.dest_node.clone(), kind, new_budget);
            for safe_path in safe_paths.iter_mut() {
                let mut path = Vec::from([node.clone()]);
                path.append(safe_path);
                result.push(path);
            }
        }
        result
    }

    fn get_all_paths(&self) -> Vec<Vec<Rc<RefCell<Node>>>> {
        let root = self.root.clone();
        let kinds = [EdgeKind::Ascending, EdgeKind::Descending];
        let budget = 1;
        let mut result: Vec<Vec<Rc<RefCell<Node>>>> = Vec::new();
        for kind in kinds {
            for path in self._get_paths(root.clone(), kind, budget) {
                result.push(path);
            }
        }
        result
    }

    fn add_level(&mut self, level: u64) {
        // add a new node
        let new_node = Rc::new(RefCell::new(Node {
            value: Some(level),
            edges: Vec::new(),
        }));
        self.nodes.push(new_node.clone());

        // attempt to establish valid edges between the prev two nodes
        // and the new node
        let start_idx = usize::try_from(0i32.max(i32::try_from(self.nodes.len()).unwrap() - 3)).unwrap();
        let new_val = new_node.borrow().value.unwrap();
        for (i, prev_node) in self.nodes[start_idx..self.nodes.len()].iter().enumerate() {

            // the cost is 1 when a level has to be skipped when creating an edge
            // upon adding the first new node is the only time when our loop
            // will have a window size of 1 (instead of 2)
            let cost = if self.nodes.len() - start_idx >= 3 {
                if i == 0 {
                    1
                } else {
                    0
                }
            } else {
                0
            };

            let mut pn = prev_node.borrow_mut();
            match pn.value {
                // links between two supplied levels
                Some(prev_val) => {
                    if prev_val < new_val && prev_val.abs_diff(new_val) <= Self::MAX_LEVEL_DIFF {
                        pn.edges.push(Rc::new(RefCell::new(Edge {
                            cost: cost,
                            dest_node: new_node.clone(),
                            kind: EdgeKind::Ascending,
                        })));
                    } else if prev_val > new_val && prev_val.abs_diff(new_val) <= Self::MAX_LEVEL_DIFF {
                        pn.edges.push(Rc::new(RefCell::new(Edge {
                            cost: cost,
                            dest_node: new_node.clone(),
                            kind: EdgeKind::Descending,
                        })));
                    }
                },
                // this is the root node only
                // always create valid links
                // need to be able to skip over the first level
                None => {
                    pn.edges.push(Rc::new(RefCell::new(Edge {
                        cost: cost,
                        dest_node: new_node.clone(),
                        kind: EdgeKind::Ascending,
                    })));
                    pn.edges.push(Rc::new(RefCell::new(Edge {
                        cost: cost,
                        dest_node: new_node.clone(),
                        kind: EdgeKind::Descending,
                    })));
                }
            }
        }
    }
}

impl Report {
    fn from(levels_input: &str) -> Self {
        let rx = Regex::new(r"\s+").expect("invalid regex");
        let levels: Vec<u64> = rx.split(levels_input).map(|s| s.parse::<u64>().unwrap()).collect();
        Self {
            levels: levels,
        }
    }

    fn is_safe(&self) -> bool {
        match get_ordering(&self.levels) {
            LevelOrdering::NotOrdered => false,
            _ => gapping_valid(&self.levels),
        }
    }

    fn is_safe_with_dampening(&self) -> bool {
        let graph = self.build_graph();
        graph.is_safe()
    }

    fn build_graph(&self) -> LevelsGraph {
        let mut graph = LevelsGraph::new();

        for level in &self.levels {
            graph.add_level(*level);
        }

        graph
    }
}


pub fn count_reports<F: Fn(Report) -> bool>(file_path: PathBuf, filter: F) -> Result<u64, Box<dyn Error>> {
    let mut safe_reports: u64 = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let report = Report::from(line);
        if filter(report) {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

pub fn count_safe_reports_strict(file_path: PathBuf) -> Result<u64, Box<dyn Error>> {
    count_reports(file_path, |report| report.is_safe())
}

pub fn count_safe_reports_dampened(file_path: PathBuf) -> Result<u64, Box<dyn Error>> {
    count_reports(file_path, |report| report.is_safe_with_dampening())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::get_test_data_path;

    #[test]
    fn test_counting() {
        let path = get_test_data_path("day2/case1.txt").unwrap();
        let result = count_safe_reports_strict(path).unwrap();
        assert_eq!(result, 2, "correctly analyzes and counts reports")
    }

    #[test]
    fn incrementing_safe() {
        let report = Report::from("1 4 6 7");
        assert!(report.is_safe(), "all incrementing 1-3");
    }

    #[test]
    fn not_all_incrementing() {
        let report = Report::from("1 4 6 5");
        assert!(!report.is_safe(), "decrementing is unsafe");
    }

    #[test]
    fn incrementing_unsafe_gap() {
        let report = Report::from("1 4 6 10");
        assert!(!report.is_safe(), "all incrementing 1-3");
    }

    #[test]
    #[should_panic]
    fn no_levels() {
        let report = Report::from("");
    }

    #[test]
    fn same_levels() {
        let report = Report::from("1 2 3 3");
        assert!(!report.is_safe(), "needs to increment or decrement by at least one");
    }

    #[test]
    fn one_level() {
        let report = Report::from("1");
        assert!(report.is_safe(), "only one level should be valid");
    }

}

#[cfg(test)]
mod tests_dampening {
    use super::*;
    use crate::common::get_test_data_path;

    #[test]
    fn test_graph_inspect() {
        let report = Report::from("1 3 2 4 5");
        let graph = report.build_graph();
        // println!("FOO");
        // println!("{}", graph);
        assert!(report.is_safe_with_dampening());

        //let all_paths = graph.get_all_paths();
        //for p in all_paths {
        //    for n in p {
        //        print!("{}\t", n.borrow().value.unwrap_or(9999));
        //    }
        //    print!("\n");
        //}
    }

    #[test]
    fn test_counting() {
        let path = get_test_data_path("day2/case1.txt").unwrap();
        let result = count_safe_reports_dampened(path).unwrap();
        assert_eq!(result, 4, "correctly analyzes and counts reports")
    }

    #[test]
    fn incrementing_safe() {
        let report = Report::from("1 4 6 7");
        assert!(report.is_safe_with_dampening(), "all incrementing 1-3");
    }

    #[test]
    fn not_all_incrementing() {
        let report = Report::from("1 4 6 5");
        assert!(report.is_safe_with_dampening(), "single value can be removed");
    }

    #[test]
    fn incrementing_unsafe_gap() {
        let report = Report::from("1 4 6 10");
        assert!(report.is_safe_with_dampening(), "can remove largest");
    }

    #[test]
    fn same_levels() {
        let report = Report::from("1 2 3 3");
        assert!(report.is_safe_with_dampening(), "same can be removed");
    }

    #[test]
    fn same_levels_multi() {
        let report = Report::from("1 2 3 3 3");
        assert!(!report.is_safe_with_dampening(), "same can be removed only once");
    }

    #[test]
    fn one_level() {
        let report = Report::from("1");
        assert!(report.is_safe_with_dampening(), "only one level should be valid");
    }

    #[test]
    fn inc_or_desc_possible() {
        let report = Report::from("2 4 3");
        assert!(report.is_safe_with_dampening(), "multiple possibilities after removal");
    }

}
