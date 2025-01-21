use std::fmt::write;
use std::fs;
use std::path::PathBuf;
use std::collections::{HashMap, VecDeque, HashSet};

pub mod constants {
    pub const INPUT_PATH: &str = "day10/input.txt";
}

type Ipos = (isize, isize);
type Upos = (usize, usize);
type Elevation = usize;
type Trail = VecDeque<Ipos>;

fn to_ipos(upos: Upos) -> Ipos {
    (isize::try_from(upos.0).unwrap(), isize::try_from(upos.1).unwrap())
}

pub struct TopoMap {
    map: HashMap<Ipos, Elevation>,
    trailheads: Vec<Ipos>,
}

impl TopoMap {

    pub fn from(input: &String) -> Self {
        let mut map: HashMap<Ipos, Elevation> = HashMap::new();
        let mut trailheads = Vec::new();
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let elevation = c.to_digit(10).unwrap() as usize;
                let pos = to_ipos((col_idx, row_idx));

                map.entry(pos).or_insert(elevation);

                if elevation == 0 {
                    trailheads.push(pos);
                }
            }
        }
        Self {
            map,
            trailheads,
        }
    }

    pub fn get_trailheads(&self) -> &Vec<Ipos> {
        &self.trailheads
    }

    pub fn get_trails(&self, start: Ipos) -> Vec<Trail> {
        let mut trails = Vec::new();
        let start_elevation = self.elevation_at(start).unwrap();
        let neighbors = self.get_neighbors(start);
        let valid_neighbors = neighbors.iter().filter(|&neighbor| {
            let elevation = self.elevation_at(*neighbor).unwrap();
            elevation == start_elevation + 1
        });
        if valid_neighbors.clone().count() == 0 {
            let end_of_trail = Trail::from([start]);
            trails.push(end_of_trail);
        } else {
            for n in valid_neighbors {
                let neighbor_trails = self.get_trails(*n);
                for mut trail in neighbor_trails {
                    trail.push_front(start);
                    trails.push(trail);
                }
            }
        }
        trails
    }

    pub fn get_neighbors(&self, pos: Ipos) -> Vec<Ipos> {
        let mut result = Vec::new();
        let neighbor_offsets: &[Ipos] = &[
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];
        for offset in neighbor_offsets.iter() {
            let neighbor_pos = (
                pos.0 + offset.0,
                pos.1 + offset.1,
            );
            match self.map.get(&neighbor_pos) {
                Some(_) =>  result.push(neighbor_pos),
                None => (),
            }
        }
        result
    }

    pub fn elevation_at(&self, pos: Ipos) -> Option<Elevation> {
        self.map.get(&pos).copied()
    }
}

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    let topo_map = TopoMap::from(&input);
    let trailheads = topo_map.get_trailheads();
    let mut result = 0;
    for th in trailheads.iter() {
        let mut distinct_endings = HashSet::new();
        let trails = topo_map.get_trails(*th);
        for t in trails.iter() {
            if t.len() == 10 {
                distinct_endings.insert(t.back().unwrap());
            }
        }
        result += distinct_endings.len();
    }
    result
}


pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> usize {
    let topo_map = TopoMap::from(&input);
    let trailheads = topo_map.get_trailheads();
    let mut result = 0;
    for th in trailheads.iter() {
        let trails = topo_map.get_trails(*th);
        for t in trails.iter() {
            if t.len() == 10 {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn get_trailheads() {
        let path = common::get_test_data_path("day10/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let map = TopoMap::from(&input);
        assert_eq!(map.get_trailheads().len(), 9);
    }

    #[test]
    fn example_day10_1() {
        let path = common::get_test_data_path("day10/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 36);
    }

    #[test]
    fn example_day10_2() {
        let path = common::get_test_data_path("day10/case2.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 2);
    }

    #[test]
    fn _example_day2() {
        let path = common::get_test_data_path("day10/case1.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 81);
    }
}
