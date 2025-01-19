use std::fs;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

pub mod constants {
    pub const INPUT_PATH: &str = "day8/input.txt";
}

type Ipos = (isize, isize);
type Upos = (usize, usize);
type Antenna = char;

struct CityMap {
    coords: HashMap<Ipos, Option<Antenna>>,
    antenna_coords: HashMap<Antenna, Vec<Ipos>>,
    num_rows: isize,
    num_cols: isize,
}

fn to_ipos(upos: Upos) -> Ipos {
    (isize::try_from(upos.0).unwrap(), isize::try_from(upos.1).unwrap())
}

fn to_isize(i: usize) -> isize {
    isize::try_from(i).unwrap()
}

impl CityMap {

    pub fn from(input: &String) -> Self {
        let mut coords: HashMap<Ipos, Option<Antenna>> = HashMap::new();
        let mut antenna_coords: HashMap<Antenna, Vec<Ipos>> = HashMap::new();

        let num_rows = to_isize(input.lines().count());
        let num_cols = to_isize(input.lines().next().unwrap().bytes().len());

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                let ipos = to_ipos((x, y));
                let c = char::try_from(c).unwrap();
                let to_insert = match c {
                    '.' => None,
                    _ => {
                        let c_coords = antenna_coords.entry(c).or_insert(Vec::new());
                        c_coords.push(ipos);
                        Some(c)
                    },
                };
                coords.entry(ipos).or_insert(to_insert);
            }
        }
        CityMap {
            coords,
            antenna_coords,
            num_rows,
            num_cols,
        }
    }

    pub fn antenna_iter(&self) -> impl Iterator<Item=&Antenna> {
        self.antenna_coords.keys()
    }

    pub fn coords_for_antenna(&self, antenna: &Antenna) -> Option<&Vec<Ipos>> {
        self.antenna_coords.get(antenna)
    }

    pub fn coord_pairs_for_antenna(&self, antenna: &Antenna) -> Option<Vec<(Ipos, Ipos)>> {
        let coords = self.coords_for_antenna(antenna);
        match coords {
            Some(coords) if coords.len() < 2 => {
                None
            },
            Some(coords) => {
                let mut result = Vec::new();
                for i in 0..coords.len()-1 {
                    for j in i+1..coords.len() {
                        result.push((coords[i], coords[j]));
                    }
                }
                Some(result)
            },
            None => None,
        }
    }

    // for a location pair, there should be two antinodes
    pub fn antinode_locations_1(&self, coord_pair: (Ipos, Ipos)) -> (Ipos, Ipos) {
        let first = coord_pair.0;
        let second = coord_pair.1;

        // create a vector
        let x_diff = second.0 - first.0;
        let y_diff = second.1 - first.1;

        let loc_1 = (second.0 + x_diff, second.1 + y_diff);
        let loc_2 = (first.0 - x_diff, first.1 - y_diff);

        (loc_1, loc_2)
    }

    pub fn antinode_locations_2(&self, coord_pair: (Ipos, Ipos)) -> Vec<Ipos> {
        let mut locations = Vec::new();

        // each pair creates antinodes at their locs
        locations.push(coord_pair.0);
        locations.push(coord_pair.1);

        let first = coord_pair.0;
        let second = coord_pair.1;

        // create a vector
        let x_diff = second.0 - first.0;
        let y_diff = second.1 - first.1;

        // inc loop
        let mut multiplier = 1;
        loop {
            let loc = (second.0 + multiplier*x_diff, second.1 + multiplier*y_diff);
            if self.contains(loc) {
                locations.push(loc);
                multiplier += 1;
            } else {
                break;
            }

        }

        multiplier = 1;
        loop {
            let loc = (first.0 - multiplier*x_diff, first.1 - multiplier*y_diff);
            if self.contains(loc) {
                locations.push(loc);
                multiplier += 1;
            } else {
                break;
            }
        }
        locations
    }

    pub fn contains(&self, pos: Ipos) -> bool {
        self.coords.contains_key(&pos)
    }
}

// any two identical characters form an antinode at
// a location where:
    // 1 antenna is twice as far as another
    // the antennas form a line

// parse the input
    // internally, hashmap<Pos, Option<char>>
// create a hashmap<char, Vec<Pos>>
// for each char
    // generate unique pairs
    // find the antinode locations for each pair
    // record them in a hashset<Pos> (solution wants unique locations)

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    let city_map = CityMap::from(input);
    let mut antinode_locations: HashSet<Ipos> = HashSet::new();

    for antenna in city_map.antenna_iter() {
        match city_map.coord_pairs_for_antenna(antenna) {
            Some(coord_pairs) => {
                for coord_pair in coord_pairs {
                    let antinodes = city_map.antinode_locations_1(coord_pair);
                    if city_map.contains(antinodes.0) {
                        antinode_locations.insert(antinodes.0);
                    }
                    if city_map.contains(antinodes.1) {
                        antinode_locations.insert(antinodes.1);
                    }
                }

            },
            None => (),
        }
    }
    antinode_locations.len()
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> usize {
    let city_map = CityMap::from(input);
    let mut antinode_locations: HashSet<Ipos> = HashSet::new();

    for antenna in city_map.antenna_iter() {
        match city_map.coord_pairs_for_antenna(antenna) {
            Some(coord_pairs) => {
                for coord_pair in coord_pairs {
                    let antinodes = city_map.antinode_locations_2(coord_pair);
                    for antinode_pos in antinodes {
                        if city_map.contains(antinode_pos) {
                            antinode_locations.insert(antinode_pos);
                        }
                    }
                }

            },
            None => (),
        }
    }
    antinode_locations.len()
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example_day8_1() {
        let path = common::get_test_data_path("day8/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_example_day8_2() {
        let path = common::get_test_data_path("day8/case1.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 34);
    }
}
