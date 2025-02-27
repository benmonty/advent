use std::fs;
use std::path::PathBuf;

use rustc_hash::{FxHashMap, FxHashSet};

enum Tile {
    Safe,
    Corrupt,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Loc {
    x: isize,
    y: isize,
}

struct MemMap {
    m: FxHashMap<Loc, Tile>,
    x_max: isize,
    y_max: isize,
    start: Loc,
    goal: Loc,
}

impl MemMap {
    pub fn new(x_max: isize, y_max: isize, corrupt_locs: &FxHashSet<Loc>) -> Self {
        let mut m = FxHashMap::<Loc, Tile>::default();
        for x in 0..=x_max {
            for y in 0..=y_max {
                let loc = Loc { x, y };
                if corrupt_locs.contains(&loc) {
                    m.entry(loc).or_insert(Tile::Corrupt);
                } else {
                    m.entry(loc).or_insert(Tile::Safe);
                }
            }
        }
        MemMap {
            m,
            x_max,
            y_max,
            start: Loc { x: 0, y: 0 },
            goal: Loc { x: x_max, y: y_max },
        } 
    }
}

pub mod constants {
    pub const INPUT_PATH: &str = "day18/input.txt";
    pub const X_MAX: isize = 70;
    pub const Y_MAX: isize = 70;
    pub const BYTES_FALLEN: isize = 1024;
}

fn parse_corrupt_locs(input: &String, mut bytes_fallen: isize) -> FxHashSet<Loc> {
    let mut result = FxHashSet::<Loc>::default();
    for line in input.lines() {
        let loc: Vec<&str> = line.split(',').collect();
        let x = loc[0].parse::<isize>().unwrap();
        let y = loc[1].parse::<isize>().unwrap();
        result.insert(Loc { x, y });
        bytes_fallen -= 1;
        if bytes_fallen == 0 {
            break;
        }
    }
    result
}

fn get_valid_moves(loc: &Loc, mmap: &MemMap) -> Vec<Loc> {
    let deltas = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];
    let mut result: Vec<Loc> = vec![];
    for d in deltas.iter() {
        let next_loc = Loc { x: loc.x + d.0, y: loc.y + d.1 };
        if next_loc.x < 0 || next_loc.y < 0 {
            continue;
        } else if next_loc.x > mmap.x_max || next_loc.y > mmap.y_max {
            continue;
        }
        match mmap.m.get(&next_loc).unwrap() {
            Tile::Safe => {
                result.push(next_loc);
            },
            Tile::Corrupt => (),
        }
    }
    result
}

fn shortest_path(mmap: &MemMap) -> Option<isize> {
    let mut best_paths = FxHashMap::<Loc, Vec<Vec<Loc>>>::default();
    let mut frontier = FxHashSet::<Loc>::default();

    let mut goal_reached = false;

    frontier.insert(mmap.start.clone());
    best_paths.entry(mmap.start.clone()).or_insert(vec![vec![]]);
    let mut added = false;

    loop {
        let mut next_frontier = FxHashSet::<Loc>::default();

        if goal_reached {
            break;
        }
        added = false;
        for frontier_loc in frontier.iter() {
            let best_len = best_paths.get(&frontier_loc).unwrap()[0].len();

            let next_locs = get_valid_moves(&frontier_loc, &mmap);

            for nl in next_locs.iter() {
                if *nl == mmap.goal {
                    goal_reached = true;
                }
                if best_paths.contains_key(&nl) {
                    let nl_best = best_paths.get(&nl).unwrap()[0].len();
                    if best_len + 1 < nl_best {
                        // new best
                        let mut updated_paths = best_paths.get(&frontier_loc).unwrap().clone();
                        for path in updated_paths.iter_mut() {
                            path.push(nl.clone());
                        }
                        best_paths.entry(nl.clone()).or_insert(updated_paths);
                        next_frontier.insert(nl.clone());
                        added = true;
                    } else if best_len + 1 == nl_best {
                        // same best
                        // for now, do nothing, just keep whatever was the current best
                        //let mut updated_paths = best_paths.get(&frontier_loc).unwrap().clone();
                        //for path in updated_paths.iter_mut() {
                        //    path.push(nl.clone());
                        //}
                        //let nl_best_paths = best_paths.get_mut(&nl).unwrap();
                        //nl_best_paths.append(&mut updated_paths);
                        //next_frontier.insert(nl.clone());
                    } else {
                        // worse path
                        // don't continue processing
                    }
                } else {
                    // new best
                    let mut updated_paths = best_paths.get(&frontier_loc).unwrap().clone();
                    for path in updated_paths.iter_mut() {
                        path.push(nl.clone());
                    }
                    best_paths.entry(nl.clone()).or_insert(updated_paths);
                    next_frontier.insert(nl.clone());
                    added = true;
                }
            }
        }
        if !added {
            break;
        }
        let best_path_locs: Vec<Loc> = best_paths.keys().map(|k| k.clone()).collect();
        for l in best_path_locs.iter() {
            if frontier.contains(&l) || next_frontier.contains(&l) {
                // do nothing
            } else {
                best_paths.remove(&l);
            }
        }
        frontier = next_frontier;
    }
    if added {
        Some(isize::try_from(best_paths.get(&mmap.goal).unwrap()[0].len()).unwrap())
    } else {
        None
    }
    /*
        best_paths: FxHashMap<Loc, Vec<Vec<Loc>>
        frontier: FxHashSet<Loc>

        let mut goal_reached = false;
        loop {
            if goal_reached {
                break
            }
            let mut next_frontier;
            for frontier_loc in frontier {
                let best = best_paths.get(frontier_loc).unwrap()[0].len()
                valid_next_locs = get_valid_moves(frontier_loc, mmap)
                for nl in valid_next_locs {
                    if no best path to nl {
                        add one
                        add nl to frontier
                    } else if best + 1 < best to nl {
                        add one
                        add nl to frontier
                    } else if best + 1 == best to nl {
                        push one
                        add nl to frontier
                    } else {
                        do nothing
                    }
                }
            }
        }
* */

}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input, constants::X_MAX, constants::Y_MAX, constants::BYTES_FALLEN)
}

pub fn _solution1(input: &String, x_max: isize, y_max: isize, bytes_fallen: isize) -> isize {
    let corrupt_locs = parse_corrupt_locs(&input, bytes_fallen);
    let mmap = MemMap::new(x_max, y_max, &corrupt_locs);
    shortest_path(&mmap).unwrap()
}

pub fn solution2(path: &PathBuf) -> String {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input, constants::X_MAX, constants::Y_MAX, constants::BYTES_FALLEN)
}

pub fn _solution2(input: &String, x_max: isize, y_max: isize, mut bytes_fallen: isize) -> String {
    loop {
        let corrupt_locs = parse_corrupt_locs(&input, bytes_fallen);
        let mmap = MemMap::new(x_max, y_max, &corrupt_locs);
        match shortest_path(&mmap) {
            None => break,
            _ => bytes_fallen += 1,
        }
    }
    let line_idx = usize::try_from(bytes_fallen - 1).unwrap();
    input.lines().collect::<Vec<_>>()[line_idx].to_string()
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common::get_test_data_path;

    #[test]
    fn day_18_1_0() {
        let bytes_fallen = 12;
        let (max_x, max_y) = (6, 6);
        let path = get_test_data_path("day18/case1.txt").unwrap();
        let input =  fs::read_to_string(path).unwrap();
        let result = _solution1(&input, max_x, max_y, bytes_fallen);
        assert_eq!(result, 22, "found min steps")
    }

    #[test]
    fn day_18_2_0() {
        let bytes_fallen = 12;
        let (max_x, max_y) = (6, 6);
        let path = get_test_data_path("day18/case1.txt").unwrap();
        let input =  fs::read_to_string(path).unwrap();
        let result = _solution2(&input, max_x, max_y, bytes_fallen);
        assert_eq!(result, "6,1", "found min steps")
    }
}
