use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub mod constants {
    pub const WORD_SEARCH_PATH: &str = "day4/word-search.txt";
    pub const NEEDLE: &str = "XMAS";
}

struct Puzzle {
    num_rows: usize,
    num_cols: usize,
    lines: Vec<Vec<char>>,
}

impl Puzzle {

    fn from(input: &str) -> Self {
        let mut lines: Vec<Vec<char>> = Vec::new();

        let num_rows = input.lines().count();
        assert_ne!(num_rows, 0, "expecting > 0 rows");

        let mut num_cols = 0;

        for line in input.lines() {
            if num_cols == 0 {
                num_cols = line.len(); // bytes, only works with ascii
                assert_ne!(num_cols, 0, "expecting > 0 cols");
            } else {
                assert_eq!(line.len(), num_cols, "found unequal row length");
            }
            let row: Vec<char> = line.as_bytes().iter().map(|b| *b as char).collect();
            lines.push(row);
        }
        Self {
            num_rows,
            num_cols,
            lines,
        }
    }

    fn at(&self, col_idx: usize, row_idx: usize) -> char {
        self.lines[row_idx][col_idx]
    }

    fn in_bounds(&self, col_idx: isize, row_idx: isize) -> bool {
        let num_cols: isize = (self.num_cols as isize).try_into().unwrap();
        let num_rows: isize = (self.num_rows as isize).try_into().unwrap();
        col_idx >= 0 && col_idx < num_cols && row_idx >= 0 && row_idx < num_rows
    }
}

fn count(puzzle: &Puzzle, col_idx: usize, row_idx: usize, col_offset: &dyn Fn(isize) -> isize, row_offset: &dyn Fn(isize) -> isize) -> usize {
    let mut eq = true;
    let icol_idx = col_idx as isize;
    let irow_idx = row_idx as isize;
    for (i, c) in constants::NEEDLE.chars().enumerate() {
        let ith_col = icol_idx + col_offset((i as isize).try_into().unwrap());
        let ith_row = irow_idx + row_offset((i as isize).try_into().unwrap());
        let ith_col_u = ith_col as usize;
        let ith_row_u = ith_row as usize;
        if !puzzle.in_bounds(ith_col, ith_row) {
            eq = false;
        } else if puzzle.at(ith_col_u, ith_row_u) != c {
            eq = false;
        }
    }
    if eq { 1 } else { 0 }
}

fn inc(i: isize) -> isize { i }
fn dec(i: isize) -> isize { -i }
fn noop(_i: isize) -> isize { 0 }

fn search_forward(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &inc;
    let row_offset = &noop;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_backward(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &dec;
    let row_offset = &noop;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_up(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &noop;
    let row_offset = &dec;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_down(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &noop;
    let row_offset = &inc;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_forward_up(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &inc;
    let row_offset = &dec;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_forward_down(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &inc;
    let row_offset = &inc;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_backward_up(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &dec;
    let row_offset = &dec;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_backward_down(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> usize {
    let col_offset = &dec;
    let row_offset = &inc;
    count(&puzzle, col_idx, row_idx, col_offset, row_offset)
}

fn search_all(puzzle: &Puzzle) -> usize {
    let mut count = 0;
    for col_idx in 0..puzzle.num_cols {
        for row_idx in 0..puzzle.num_rows {
            count +=
                search_forward(puzzle, col_idx, row_idx)
                + search_backward(puzzle, col_idx, row_idx)
                + search_up(puzzle, col_idx, row_idx)
                + search_down(puzzle, col_idx, row_idx)
                + search_forward_up(puzzle, col_idx, row_idx)
                + search_forward_down(puzzle, col_idx, row_idx)
                + search_backward_up(puzzle, col_idx, row_idx)
                + search_backward_down(puzzle, col_idx, row_idx);
        }
    }
    count
}

struct Target(isize, isize, char);

fn matches_target(puzzle: &Puzzle, col_idx: usize, row_idx: usize, target: &Target) -> bool {

    let mut found = true;
    let target_col = isize::try_from(col_idx).unwrap() + target.0;
    let target_row = isize::try_from(row_idx).unwrap() + target.1;

    if !puzzle.in_bounds(target_col, target_row) {
        found = false;
    } else {
        let target_col = (target_col as usize).try_into().unwrap();
        let target_row = (target_row as usize).try_into().unwrap();
        if puzzle.at(target_col, target_row) != target.2 {
            found = false;
        }
    }
    found
}

fn found_all_targets(puzzle: &Puzzle, col_idx: usize, row_idx: usize, targets: &Vec<Target>) -> bool {
    targets.iter().all(|target| matches_target(puzzle, col_idx, row_idx, &target))
}

fn found_any_target_group(puzzle: &Puzzle, col_idx: usize, row_idx: usize, target_groups: &Vec<Vec<Target>>) -> bool {
    target_groups.iter().any(|group| found_all_targets(puzzle, col_idx, row_idx, &group))
}

fn has_vertical(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> bool {
    let verticals = vec![
        vec![
            Target(0, -1, 'M'),
            Target(0, 0, 'A'),
            Target(0, 1, 'S'),
        ],
        vec![
            Target(0, 1, 'M'),
            Target(0, 0, 'A'),
            Target(0, -1, 'S'),

        ],
    ];
    found_any_target_group(puzzle, col_idx, row_idx, &verticals)
}

fn has_horizontal(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> bool {
    let horizontals = vec![
        vec![
            Target(-1, 0, 'M'),
            Target(0, 0, 'A'),
            Target(1, 0, 'S'),
        ],
        vec![
            Target(1, 0, 'M'),
            Target(0, 0, 'A'),
            Target(-1, 0, 'S'),

        ],
    ];
    found_any_target_group(puzzle, col_idx, row_idx, &horizontals)
}

fn has_pos_diag(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> bool {
    let pos_diag = vec![
        vec![
            Target(-1, -1, 'M'),
            Target(0, 0, 'A'),
            Target(1, 1, 'S'),
        ],
        vec![
            Target(1, 1, 'M'),
            Target(0, 0, 'A'),
            Target(-1, -1, 'S'),

        ],
    ];
    found_any_target_group(puzzle, col_idx, row_idx, &pos_diag)
}

fn has_neg_diag(puzzle: &Puzzle, col_idx: usize, row_idx: usize) -> bool {
    let neg_diag = vec![
        vec![
            Target(-1, 1, 'M'),
            Target(0, 0, 'A'),
            Target(1, -1, 'S'),
        ],
        vec![
            Target(1, -1, 'M'),
            Target(0, 0, 'A'),
            Target(-1, 1, 'S'),

        ],
    ];
    found_any_target_group(puzzle, col_idx, row_idx, &neg_diag)
}

fn _log_cross(puzzle: &Puzzle, col_idx: usize, row_idx: usize) {
    for row_idx in row_idx - 1..=row_idx + 1 {
        for col_idx in col_idx - 1..=col_idx + 1 {
            let r_idx = usize::try_from(row_idx).unwrap();
            let c_idx = usize::try_from(col_idx).unwrap();
            print!("{}", puzzle.at(c_idx, r_idx));
        }
        print!("\n");
    }
}


fn search_all_crosses(puzzle: &Puzzle) -> usize {
    let mut count = 0;
    for col_idx in 0..puzzle.num_cols {
        for row_idx in 0..puzzle.num_rows {
            if has_neg_diag(puzzle, col_idx, row_idx) && has_pos_diag(puzzle, col_idx, row_idx) {
                count += 1;
            }
        }
    }
    count
}

pub fn count_xmas(word_search: PathBuf) -> Result<usize, Box<dyn Error>> {
    let puzzle = fs::read_to_string(word_search)?;
    Ok(count_xmas_str(&puzzle))
}

pub fn count_xmas_str(puzzle: &str) -> usize {
    let puzzle = Puzzle::from(puzzle);
    search_all(&puzzle)
}

pub fn count_crosses(word_search: PathBuf) -> Result<usize, Box<dyn Error>> {
    let puzzle = fs::read_to_string(word_search)?;
    Ok(count_crosses_str(&puzzle))
}

pub fn count_crosses_str(puzzle: &str) -> usize {
    let puzzle = Puzzle::from(puzzle);
    search_all_crosses(&puzzle)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn test_xmas_count() {
        let path = common::get_test_data_path("day4/case1.txt").unwrap();
        let result = count_xmas(path).unwrap();
        assert_eq!(result, 18, "correctly analyzes and counts reports")
    }

    #[test]
    fn test_all_directions() {
        let path = common::get_test_data_path("day4/case2.txt").unwrap();
        let result = count_xmas(path).unwrap();
        assert_eq!(result, 8, "correctly analyzes and counts reports")
    }

    #[test]
    fn test_all_boundaries() {
        let path = common::get_test_data_path("day4/case3.txt").unwrap();
        let result = count_xmas(path).unwrap();
        assert_eq!(result, 16, "correctly analyzes and counts reports")
    }

    #[test]
    fn test_crosses_count() {
        let path = common::get_test_data_path("day4/case1.txt").unwrap();
        let result = count_crosses(path).unwrap();
        assert_eq!(result, 9, "correctly analyzes and counts reports")
    }

    #[test]
    fn test_crosses_count_diagonals() {
        let path = common::get_test_data_path("day4/case4.txt").unwrap();
        let result = count_crosses(path).unwrap();
        assert_eq!(result, 4, "correctly analyzes and counts reports")
    }

    #[test]
    fn test_crosses_count_verticals() {
        let path = common::get_test_data_path("day4/case5.txt").unwrap();
        let result = count_crosses(path).unwrap();
        assert_eq!(result, 0, "correctly analyzes and counts reports")
    }

    #[test]
    fn test_crosses_count_combined() {
        let path = common::get_test_data_path("day4/case6.txt").unwrap();
        let result = count_crosses(path).unwrap();
        assert_eq!(result, 4, "correctly analyzes and counts reports")
    }

}
