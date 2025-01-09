use std::path::PathBuf;
use std::error::Error;
use std::fs;
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
}


pub fn count_safe_reports(file_path: PathBuf) -> Result<u64, Box<dyn Error>> {
    let mut safe_reports: u64 = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let report = Report::from(line);
        if report.is_safe() {
            safe_reports += 1;
        }
    }
    Ok(safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::get_test_data_path;

    #[test]
    fn test_counting() {
        let path = get_test_data_path("day2/case1.txt").unwrap();
        let result = count_safe_reports(path).unwrap();
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
