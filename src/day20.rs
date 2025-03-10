use std::fs;
use std::path::PathBuf;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Loc {
    x: isize,
    y: isize,
}

pub struct Dimensions {
    width: isize,
    height: isize,
}

pub enum Tile {
    Vacant,
    Wall,
}

pub mod constants {
    pub const INPUT_PATH: &str = "day20/input.txt";
}

pub struct Track {
    m: FxHashMap<Loc, Tile>,
    start: Loc,
    end: Loc,
    dim: Dimensions,
}

impl Track {
    pub fn from(input: &String) -> Self {
        let mut y = 0;
        let mut m: FxHashMap<Loc, Tile> = FxHashMap::default();
        let mut start = Loc { x: -1, y: -1 };
        let mut end = Loc { x: -1, y: -1 };
        for row in input.lines() {
            let mut x = 0;
            for c in row.chars() {
                let loc = Loc { x, y };
                match c {
                    '#' => m.entry(loc.clone()).or_insert(Tile::Wall),
                    _ => m.entry(loc.clone()).or_insert(Tile::Vacant),
                };
                match c {
                    'S' => start = loc.clone(),
                    'E' => end = loc.clone(),
                    _ => (),
                };
                x += 1;
            }
            y += 1;
        }
        let width = isize::try_from(input.lines().next().unwrap().len()).unwrap();
        let height = isize::try_from(input.lines().count()).unwrap();
        let dim = Dimensions { width, height };
        Self {
            m,
            start,
            end,
            dim,
        }
    }
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input, 100)
}

pub fn get_next_moves(track: &Track, pos: &Loc) -> Vec<Loc> {
    let offsets = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
    ];

    let mut result = vec![];

    for o in offsets.iter() {
        let l = Loc { x: pos.x + o.0, y: pos.y + o.1 };
        match track.m.get(&l) {
            Some(Tile::Vacant) => result.push(l),
            Some(Tile::Wall) => (),
            None => (),
        };
    }

    result
}

pub fn get_cheat_offsets(pico_s: isize) -> FxHashSet<(isize, isize)> {
    let mut result = FxHashSet::default();
    for x in -pico_s..=pico_s {
        for y in -pico_s..=pico_s {
            if x.abs() + y.abs() == pico_s {
                result.insert((x, y));
            }
        }
    }
    result
}

pub fn compute_goal_distances(track: &Track) -> FxHashMap<Loc, isize> {
    let mut result = FxHashMap::default();
    let mut pos = track.end.clone();
    let mut dist = 0;

    loop {
        result.entry(pos.clone()).or_insert(dist);
        if pos == track.start {
            break;
        }
        let moves = get_next_moves(track, &pos);

        let mut move_count = 0;
        let mut already_moved_count = 0;
        for m in moves.iter() {
            if result.contains_key(&m) {
                assert!(already_moved_count == 0);
                already_moved_count += 1;
            } else {
                assert!(move_count == 0);
                pos = m.clone();
                dist += 1;
                move_count += 1;
            }
        }
        assert!(move_count == 1);
    }

    result
}

pub fn _solution1(input: &String, min_savings_ps: isize) -> isize {
    let track = Track::from(&input);
    let dists = compute_goal_distances(&track);
    let cheats = get_cheats(&track, &dists, 2);

    let mut result = 0;
    for (ps, count) in cheats.iter() {
        if *ps >= min_savings_ps {
            result += count;
        }
    }
    result
}

pub fn get_cheats(track: &Track, dists: &FxHashMap<Loc, isize>, cheat_ps: isize) -> FxHashMap<isize, isize> {
    let offsets = get_cheat_offsets(cheat_ps);
    let mut result = FxHashMap::default();
    let start_cost_remaining = dists.get(&track.start).unwrap();
    for (pos, pos_cost_remaining) in dists.iter() {
        for o in offsets.iter() {
            let cheat_dest = Loc { x: pos.x + o.0, y: pos.y + o.1 };
            match track.m.get(&cheat_dest) {
                Some(Tile::Vacant) => {
                    // test for savings
                    let steps_taken = start_cost_remaining - pos_cost_remaining;
                    let cheat_cost_remaining = dists.get(&cheat_dest).unwrap();
                    let cheat_cost = steps_taken + cheat_cost_remaining + cheat_ps;
                    let savings = start_cost_remaining - cheat_cost;
                    if  savings > 0 {
                        result.entry(savings).or_insert(0);
                        *result.get_mut(&savings).unwrap() += 1;
                    }
                },
                Some(Tile::Wall) => {
                    // invalid cheat
                },
                None => {
                    // off the map
                },
            };
        }
    }
    result
}

pub fn solution2(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input, 100)
}

pub fn _solution2(input: &String, min_savings_ps: isize) -> isize {
    let track = Track::from(&input);
    let dists = compute_goal_distances(&track);
    let cheats = get_cheats2(&track, &dists, 20);

    let mut result = 0;
    for (ps, count) in cheats.iter() {
        if *ps >= min_savings_ps {
            result += count;
        }
    }
    result
}

pub fn get_cheat_offsets2(pico_s: isize) -> FxHashSet<(isize, isize)> {
    let mut result = FxHashSet::default();
    for x in -pico_s..=pico_s {
        for y in -pico_s..=pico_s {
            if x.abs() + y.abs() <= pico_s {
                result.insert((x, y));
            }
        }
    }
    result
}

pub fn get_cheats2(track: &Track, dists: &FxHashMap<Loc, isize>, cheat_ps: isize) -> FxHashMap<isize, isize> {
    let offsets = get_cheat_offsets2(cheat_ps);
    let mut result = FxHashMap::default();
    let start_cost_remaining = dists.get(&track.start).unwrap();
    for (pos, pos_cost_remaining) in dists.iter() {
        for o in offsets.iter() {
            let cheat_dest = Loc { x: pos.x + o.0, y: pos.y + o.1 };
            match track.m.get(&cheat_dest) {
                Some(Tile::Vacant) => {
                    // test for savings
                    let steps_taken = start_cost_remaining - pos_cost_remaining;
                    let cheat_cost_remaining = dists.get(&cheat_dest).unwrap();
                    let cheat_cost = steps_taken + cheat_cost_remaining + (o.0.abs() + o.1.abs());
                    let savings = start_cost_remaining - cheat_cost;
                    if  savings > 0 {
                        result.entry(savings).or_insert(0);
                        *result.get_mut(&savings).unwrap() += 1;
                    }
                },
                Some(Tile::Wall) => {
                    // invalid cheat
                },
                None => {
                    // off the map
                },
            };
        }
    }
    result
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn day_20_1_1() {
        let path = common::get_test_data_path("day20/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let track = Track::from(&input);
        let dists = compute_goal_distances(&track);
        let cheats = get_cheats(&track, &dists, 2);
        let expected: FxHashMap<isize, isize> = Vec::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]).into_iter().collect();
        assert_eq!(
            cheats,
            expected,
        );
    }

    #[test]
    fn day_20_1_2() {
        let path = common::get_test_data_path("day20/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let result = _solution1(&input, 10);
        assert_eq!(result, 10);
    }

    #[test]
    fn day_20_2_1() {
        let path = common::get_test_data_path("day20/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let track = Track::from(&input);
        let dists = compute_goal_distances(&track);
        let cheats = get_cheats2(&track, &dists, 20);
        let mut gte50_savings_cheats = FxHashMap::default();
        for (ps, count) in cheats.into_iter() {
            if ps >= 50 {
                gte50_savings_cheats.entry(ps).or_insert(count);
            }
        }
        let expected: FxHashMap<isize, isize> = Vec::from([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ]).into_iter().collect();
        assert_eq!(
            gte50_savings_cheats,
            expected,
        );
    }

    #[test]
    fn day_20_2_2() {
        let path = common::get_test_data_path("day20/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let result = _solution2(&input, 74);
        assert_eq!(result, 7);
    }
}
