use std::fs;
use std::path::PathBuf;
use rustc_hash::{FxHashMap, FxHashSet};

pub mod constants {
    pub const INPUT_PATH: &str = "day12/input.txt";
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Icoord {
    x: isize,
    y: isize,
}

impl Icoord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

type PlantType = char;

pub struct Region {
    plant_type: PlantType,
    plots: FxHashSet<Icoord>,
}

impl Region {
    pub fn get_area(&self) -> usize {
        self.plots.len()
    }

    pub fn get_perimeter(&self) -> usize {
        let mut permiter = 0;

        for region_coord in self.plots.iter() {
            for n in get_neighboring_coords(region_coord) {
                match self.plots.get(&n) {
                    Some(_) => (),
                    None => permiter += 1,
                }
            }
        }
        permiter
    }

    pub fn get_cost(&self) -> usize {
        self.get_area() * self.get_perimeter()
    }
}

pub struct Garden {
    plots: FxHashMap<Icoord, PlantType>,
    width: isize,
    height: isize,
}

fn get_neighboring_coords(c: &Icoord) -> Vec<Icoord> {
    let offsets = [
        Icoord::new(0, 1),
        Icoord::new(0, -1),
        Icoord::new(1, 0),
        Icoord::new(-1, 0),
    ];

    offsets.map(|o| {
        Icoord {
            x: c.x + o.x,
            y: c.y + o.y,
        }
    }).into_iter().collect()
}

impl Garden {

    pub fn from(input: &String) -> Self {
        let mut plots = FxHashMap::default();

        let height = isize::try_from(
            input.lines().count()
        ).unwrap();

        let width = isize::try_from(
            input.lines().next().unwrap().len()
        ).unwrap();

        for (y, line) in input.lines().enumerate() {
            let y = isize::try_from(y).unwrap();

            for (x, c) in line.chars().enumerate() {

                let x = isize::try_from(x).unwrap();
                let coord = Icoord { x, y };

                plots.entry(coord).or_insert(c);
            }
        }

        Self {
            plots,
            width,
            height,
        }
    }

    pub fn _get_matching_neighbors(&self, coord: &Icoord) -> Vec<Icoord> {
        let mut result = vec![];

        let plot = self.plots.get(coord).unwrap();

        for neighbor_coord in get_neighboring_coords(coord) {
            match self.plots.get(&neighbor_coord) {
                Some(neighbor) => {
                    if neighbor == plot {
                        result.push(neighbor_coord)
                    }
                },
                None => (),
            }
        }

        result
    }

    pub fn _get_region_plots(&self, coord: &Icoord) -> Vec<Icoord> {
        let mut in_region: FxHashSet<Icoord> = FxHashSet::default();
        let mut to_examine: Vec<Icoord> = vec![coord.clone()];

        while to_examine.len() != 0 {
            let coord = to_examine.pop().unwrap();
            in_region.insert(coord.clone());
            let neighbor_coords = self._get_matching_neighbors(&coord);

            for nc in neighbor_coords {
                if !in_region.contains(&nc) {
                    to_examine.push(nc.clone());
                    in_region.insert(nc);
                }
            }
        }

        in_region.into_iter().collect()
    }

    pub fn get_regions(&self) -> Vec<Region> {
        let mut result: Vec<Region> = vec![];
        let mut in_region: FxHashSet<Icoord> = FxHashSet::default();

        for x in 0..self.width {
            for y in 0..self.height {
                let coord = Icoord { x, y, };
                if !in_region.contains(&coord) {
                    in_region.insert(coord.clone());
                    let plots = self._get_region_plots(&coord);
                    for region_coord in plots.iter() {
                        in_region.insert(region_coord.clone());
                    }
                    let region = Region {
                        plots: plots.into_iter().collect(),
                        plant_type: *self.plots.get(&coord).unwrap(),
                    };
                    result.push(region);
                }
            }
        }
        result
    }
}

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    let garden = Garden::from(&input);
    let mut cost = 0;
    for region in garden.get_regions().iter() {
        cost += region.get_cost()
    }
    cost
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> usize {
    0
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example_day12_1() {
        let path = common::get_test_data_path("day12/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_example_day2() {
        assert!(false, "todo")
    }
}
