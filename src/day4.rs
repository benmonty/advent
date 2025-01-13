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

pub fn count_xmas(memory_file: PathBuf) -> Result<usize, Box<dyn Error>> {
    let puzzle = fs::read_to_string(memory_file)?;
    Ok(count_xmas_str(&puzzle))
}

pub fn count_xmas_str(puzzle: &str) -> usize {
    let puzzle = Puzzle::from(puzzle);
    search_all(&puzzle)
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

}
