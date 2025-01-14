use std::error::Error;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;

pub mod constants {
    pub const WORD_SEARCH_PATH: &str = "day4/word-search.txt";
    pub const NEEDLE: &str = "XMAS";
}

#[derive(Copy, Clone)]
struct ColIdx(isize);

impl Add for ColIdx {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}
impl Add<isize> for ColIdx {
    type Output = ColIdx;

    fn add(self, other: isize) -> Self {
        Self(self.0 + other)
    }
}


#[derive(Copy, Clone)]
struct RowIdx(isize);

impl Add for RowIdx {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<isize> for RowIdx {
    type Output = RowIdx;

    fn add(self, other: isize) -> Self {
        Self(self.0 + other)
    }
}

#[derive(Copy, Clone)]
struct Coord {
    col: ColIdx,
    row: RowIdx,
}

impl Coord {
    fn from(col: isize, row: isize) -> Self {
        Self {
            col: ColIdx(col),
            row: RowIdx(row),
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

#[derive(Copy, Clone)]
struct Query {
    coord: Coord,
    value: char,
}

impl Query {

    fn from(coord: &Coord, value: char) -> Self {
        Self {
            coord: *coord,
            value,
        }
    }
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

    fn find_all(&self, queries: &Vec<Query>) -> bool {
        queries.iter().all(|q| self.find(q))
    }

    fn find(&self, query: &Query) -> bool {
        match self.at(&query.coord) {
            Some(c) => {
                c == query.value
            },
            None => false,
        }
    }

    fn at(&self, coord: &Coord) -> Option<char> {
        if self._in_bounds(&coord) {
            let col = usize::try_from(coord.col.0).unwrap();
            let row = usize::try_from(coord.row.0).unwrap();
            Some(self.lines[row][col])
        } else {
            None
        }
    }

    fn _in_bounds(&self, coord: &Coord) -> bool {
        let col_idx = coord.col.0;
        let row_idx = coord.row.0;
        let num_cols = isize::try_from(self.num_cols).unwrap();
        let num_rows = isize::try_from(self.num_rows).unwrap();

        col_idx >= 0 && col_idx < num_cols && row_idx >= 0 && row_idx < num_rows
    }
}


fn build_queries(base: &Coord, col_offset: &dyn Fn(isize) -> isize, row_offset: &dyn Fn(isize) -> isize) -> Vec<Query> {
    let mut queries = Vec::new();
    for (i, c) in constants::NEEDLE.chars().enumerate() {
        let i = isize::try_from(i).unwrap();
        let coord = Coord {
            col: base.col + col_offset(i),
            row: base.row + row_offset(i),
        };
        let q = Query {
            coord,
            value: c,
        };
        queries.push(q);
    }
    queries
}


fn inc(i: isize) -> isize { i }
fn dec(i: isize) -> isize { -i }
fn noop(_i: isize) -> isize { 0 }


fn count_forward(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &inc;
    let row_offset = &noop;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_backward(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &dec;
    let row_offset = &noop;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_up(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &noop;
    let row_offset = &dec;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_down(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &noop;
    let row_offset = &inc;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_forward_up(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &inc;
    let row_offset = &dec;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_forward_down(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &inc;
    let row_offset = &inc;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_backward_up(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &dec;
    let row_offset = &dec;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_backward_down(puzzle: &Puzzle, base: &Coord) -> usize {
    let col_offset = &dec;
    let row_offset = &inc;
    let queries = build_queries(base, col_offset, row_offset);
    match puzzle.find_all(&queries) {
        true => 1,
        false => 0,
    }
}

fn count_all(puzzle: &Puzzle) -> usize {
    let mut count = 0;
    for col in 0..puzzle.num_cols {
        let col = ColIdx(isize::try_from(col).unwrap());
        for row in 0..puzzle.num_rows {
            let row = RowIdx(isize::try_from(row).unwrap());
            let base = Coord { row, col };
            count +=
                count_forward(&puzzle, &base)
                + count_backward(&puzzle, &base)
                + count_up(puzzle, &base)
                + count_down(puzzle, &base)
                + count_forward_up(puzzle, &base)
                + count_forward_down(puzzle, &base)
                + count_backward_up(puzzle, &base)
                + count_backward_down(puzzle, &base);
        }
    }
    count
}

fn find_any_query_group(puzzle: &Puzzle, query_groups: &Vec<Vec<Query>>) -> bool {
    query_groups.iter().any(|group| puzzle.find_all(&group))
}

fn has_pos_diag(puzzle: &Puzzle, base: &Coord) -> bool {
    let pos_diag = vec![
        vec![
            Query {
                coord: *base + Coord::from(-1, -1),
                value: 'M',
            },
            Query {
                coord: *base + Coord::from(0, 0),
                value: 'A',
            },
            Query {
                coord: *base + Coord::from(1, 1),
                value: 'S',
            },
        ],
        vec![
            Query {
                coord: *base + Coord::from(-1, -1),
                value: 'S',
            },
            Query {
                coord: *base + Coord::from(0, 0),
                value: 'A',
            },
            Query {
                coord: *base + Coord::from(1, 1),
                value: 'M',
            },
        ],
    ];
    find_any_query_group(puzzle, &pos_diag)
}

fn has_neg_diag(puzzle: &Puzzle, base: &Coord) -> bool {
    let neg_diag = vec![
        vec![
            Query {
                coord: *base + Coord::from(-1, 1),
                value: 'M',
            },
            Query {
                coord: *base + Coord::from(0, 0),
                value: 'A',
            },
            Query {
                coord: *base + Coord::from(1, -1),
                value: 'S',
            },
        ],
        vec![
            Query {
                coord: *base + Coord::from(-1, 1),
                value: 'S',
            },
            Query {
                coord: *base + Coord::from(0, 0),
                value: 'A',
            },
            Query {
                coord: *base + Coord::from(1, -1),
                value: 'M',
            },
        ],
    ];
    find_any_query_group(puzzle, &neg_diag)
}

//fn _log_cross(puzzle: &Puzzle, col_idx: usize, row_idx: usize) {
//    for row_idx in row_idx - 1..=row_idx + 1 {
//        for col_idx in col_idx - 1..=col_idx + 1 {
//            let r_idx = usize::try_from(row_idx).unwrap();
//            let c_idx = usize::try_from(col_idx).unwrap();
//            print!("{}", puzzle.at(c_idx, r_idx));
//        }
//        print!("\n");
//    }
//}


fn get_x_count(puzzle: &Puzzle) -> usize {
    let mut count = 0;
    for col in 0..puzzle.num_cols {
        let col = ColIdx(isize::try_from(col).unwrap());
        for row in 0..puzzle.num_rows {
            let row = RowIdx(isize::try_from(row).unwrap());
            let base = Coord { col, row };
            if has_neg_diag(&puzzle, &base) && has_pos_diag(&puzzle, &base) {
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
    count_all(&puzzle)
}

pub fn count_crosses(word_search: PathBuf) -> Result<usize, Box<dyn Error>> {
    let puzzle = fs::read_to_string(word_search)?;
    Ok(count_crosses_str(&puzzle))
}

pub fn count_crosses_str(puzzle: &str) -> usize {
    let puzzle = Puzzle::from(puzzle);
    get_x_count(&puzzle)
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
