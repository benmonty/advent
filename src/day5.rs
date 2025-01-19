use std::collections::HashMap;
use std::path::PathBuf;
use std::slice::Iter;
use std::fmt;
use std::fs;
use std::cmp::Ordering;
use std::cmp;
use std::rc::Rc;

pub mod constants {
    pub const PRINTER_UPDATES: &str = "day5/ordering-updates.txt";
}

pub struct OrderRule {
    before: usize,
    after: usize,
}

impl fmt::Display for OrderRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OrderRule(before:{}, after:{})", self.before, self.after)
    }
}

pub struct OrderingRules {
    rules: Vec<Rc<OrderRule>>,
    rules_map: HashMap<(usize, usize), Rc<OrderRule>>,
}

impl OrderingRules {

    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            rules_map: HashMap::new(),
        }
    }

    pub fn from_str(&mut self, line: &str) {
        let page_numbers: Vec<usize> = line.split("|").map(|digits| digits.parse::<usize>().unwrap()).collect();
        let rule = Rc::new(OrderRule {
            before: page_numbers[0],
            after: page_numbers[1],
        });
        self.rules.push(rule.clone());
        let min = cmp::min(rule.before, rule.after);
        let max = cmp::max(rule.before, rule.after);
        self.rules_map.entry((min, max)).or_insert(rule.clone());
    }

    pub fn iter(&self) -> Iter<Rc<OrderRule>> {
        self.rules.iter()
    }

    pub fn contains(&self, key: (usize, usize)) -> bool {
        self.rules_map.contains_key(&key)
    }

    pub fn get_comparator<'a>(&'a self) -> Box<dyn FnMut(&usize, &usize) -> Ordering + 'a> {
        Box::new(move |a: &usize, b: &usize| {
            let min = cmp::min(a, b);
            let max = cmp::max(a, b);
            let order_rule = self.rules_map.get(&(*min, *max)).unwrap();
            if *a == *b {
                Ordering::Equal
            } else if order_rule.after == *a {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
    }
}

impl fmt::Display for OrderingRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rule in self.rules.iter() {
            writeln!(f, "{}", rule).unwrap();
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Update {
    page_numbers: Vec<usize>,
}

impl Update {

    pub fn get_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for i in 0..self.page_numbers.len()-1 {
            for j in i+1..self.page_numbers.len() {
                let first = self.page_numbers[i].clone();
                let second = self.page_numbers[j].clone();
                let min = cmp::min(first, second);
                let max = cmp::max(first, second);
                pairs.push((min, max));
            }
        }
        pairs
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Update[").unwrap();
        write!(f, "{}", self.page_numbers.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")).unwrap();
        write!(f, "]")
    }
}

pub struct PrintUpdates {
    updates: Vec<Update>,
}

impl PrintUpdates {
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
        }
    }

    pub fn from_str(&mut self, line: &str) {
        let page_numbers: Vec<usize> = line.split(",").map(|digits| digits.parse::<usize>().unwrap()).collect();
        self.updates.push(Update { page_numbers });
    }

    pub fn add(&mut self, update: Update) {
        self.updates.push(update);
    }

    pub fn iter(&self) -> Iter<Update> {
        self.updates.iter()
    }

    pub fn at(&self, idx: usize) -> &Update {
        &self.updates[idx]
    }
}

impl fmt::Display for PrintUpdates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for update in self.updates.iter() {
            writeln!(f, "{}", update).unwrap();
        }
        Ok(())
    }
}


pub struct PrintInstructions {
    pub rules: OrderingRules,
    pub updates: PrintUpdates,
}

impl PrintInstructions {

    pub fn from(raw_details: &String) -> Self {
        let mut rules = OrderingRules::new();
        let mut updates = PrintUpdates::new();

        let mut break_encountered = false;
        for line in raw_details.lines() {
            if line == "" {
                break_encountered = true;
            } else if break_encountered {
                updates.from_str(line);
            } else {
                rules.from_str(line);
            }
        }
        Self {
            rules,
            updates,
        }
    }

    pub fn get_ordered_update(&self, update: &Update) -> Update {
        let mut page_numbers = update.page_numbers.clone();
        page_numbers.sort_by(self.rules.get_comparator());
        Update {
            page_numbers,
        }
    }

    pub fn print(&self) {
        println!("{}", self.rules.to_string());
        println!("{}", self.updates.to_string());
    }
}

pub fn compute_part1_solution(path: &PathBuf) -> usize {
    let raw_details =  fs::read_to_string(path).unwrap();
    _compute_part1_solution(&raw_details)
}

pub fn _compute_part1_solution(raw_details: &String) -> usize {
    let instructions = PrintInstructions::from(&raw_details);

    let mut correct_updates: Vec<&Update> = Vec::new();
    for update in instructions.updates.iter() {
        let expected_update = instructions.get_ordered_update(&update);
        if update.page_numbers == expected_update.page_numbers {
            correct_updates.push(&update);
        }
    }
    let mut result = 0;
    for update in correct_updates {
        let num_pages = update.page_numbers.len();
        result += update.page_numbers[num_pages/2];
    }
    result
}

pub fn compute_part2_solution(path: &PathBuf) -> usize {
    let raw_details =  fs::read_to_string(path).unwrap();
    _compute_part2_solution(&raw_details)
}

pub fn _compute_part2_solution(raw_details: &String) -> usize {
    let instructions = PrintInstructions::from(&raw_details);

    let mut corrected_updates: Vec<Update> = Vec::new();
    for update in instructions.updates.iter() {
        let expected_update = instructions.get_ordered_update(&update);
        if update.page_numbers != expected_update.page_numbers {
            corrected_updates.push(expected_update);
        }
    }
    let mut result = 0;
    for update in corrected_updates {
        let num_pages = update.page_numbers.len();
        result += update.page_numbers[num_pages/2];
    }
    result
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example() {
        let path = common::get_test_data_path("day5/case1.txt").unwrap();
        let result = compute_part1_solution(&path);
        assert_eq!(result, 143, "computes ordering and sums correctly pt1");
    }

    #[test]
    fn test_example_day2() {
        let path = common::get_test_data_path("day5/case1.txt").unwrap();
        let result = compute_part2_solution(&path);
        assert_eq!(result, 123, "computes ordering and sums correctly pt2");
    }
}

