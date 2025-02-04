use std::fs;
use std::path::PathBuf;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::cmp::min;

pub mod constants {
    pub const INPUT_PATH: &str = "day14/input.txt";
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Location {
    x: isize,
    y: isize,
}

pub struct Dimensions {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Robot {
    loc: Location,
    vel: Velocity,
}

pub struct Lobby {
    dimensions: Dimensions,
    robots: Vec<Robot>,
}

impl Lobby {

    pub fn new(dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            robots: vec![],
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct BoundingBox {
    upper_left: Location,
    lower_right: Location,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Quadrant {
    One,
    Two,
    Three,
    Four
}

const QUADRANTS: [Quadrant; 4] = [
    Quadrant::One,
    Quadrant::Two,
    Quadrant::Three,
    Quadrant::Four,
];

pub fn in_bounds(l: &Location, bb: &BoundingBox) -> bool {
    l.x >= bb.upper_left.x && l.x <= bb.lower_right.x
        && l.y >= bb.upper_left.y && l.y <= bb.lower_right.y
}

impl Lobby {

    pub fn get_next_loc(&self, robot: &Robot) -> Location {
        let try_next_x = robot.loc.x + robot.vel.x;
        let try_next_y = robot.loc.y + robot.vel.y;
        let next_x;
        let next_y;

        if try_next_x >= 0 {
            next_x = ((robot.loc.x + robot.vel.x) % self.dimensions.x).abs();
        } else {
            next_x = self.dimensions.x - ((robot.loc.x + robot.vel.x) % self.dimensions.x).abs();
        }

        if try_next_y >= 0 {
            next_y = ((robot.loc.y + robot.vel.y) % self.dimensions.y).abs();
        } else {
            next_y = self.dimensions.y - ((robot.loc.y + robot.vel.y) % self.dimensions.y).abs();
        }

        //println!("X: ({}+{}) % {} -> {}", robot.loc.x, robot.vel.x, self.dimensions.x, next_x);
        //println!("Y: ({}+{}) % {} -> {}", robot.loc.y, robot.vel.y, self.dimensions.y, next_y);
        Location {
            x: next_x,
            y: next_y,
        }
    }

    pub fn get_quadrant_bounds(&self) -> FxHashMap<Quadrant, BoundingBox> {
        let mut result = FxHashMap::default();

        let min_x = 0;
        let min_y = 0;
        let max_x = self.dimensions.x - 1;
        let max_y = self.dimensions.y - 1;
        let mid_x = self.dimensions.x/2;
        let mid_y = self.dimensions.y/2;

        result.entry(Quadrant::One).or_insert(BoundingBox {
            upper_left: Location { x: min_x, y: min_y },
            lower_right: Location { x: mid_x - 1, y: mid_y - 1 },
        });

        result.entry(Quadrant::Two).or_insert(BoundingBox {
            upper_left: Location { x: min_x, y: mid_y + 1 },
            lower_right: Location { x: mid_x - 1, y: max_y },
        });

        result.entry(Quadrant::Three).or_insert(BoundingBox {
            upper_left: Location { x: mid_x + 1, y: min_y  },
            lower_right: Location { x: max_x, y: mid_y - 1 },
        });

        result.entry(Quadrant::Four).or_insert(BoundingBox {
            upper_left: Location { x: mid_x + 1, y: mid_y + 1  },
            lower_right: Location { x: max_x, y: max_y },
        });

        result
    }

    pub fn _get_loc_robot_counts(&self) -> FxHashMap<Location, isize> {
        let mut robot_map: FxHashMap<Location, isize> = FxHashMap::default();

        for robot in self.robots.iter() {
            robot_map.entry(robot.loc.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        robot_map
    }

    pub fn get_quadrant_scores(&self) -> FxHashMap<Quadrant, isize> {
        let mut result: FxHashMap<Quadrant, isize> = FxHashMap::default();
        let all_bounds = self.get_quadrant_bounds();

        let robot_map = self._get_loc_robot_counts();

        for (loc, count) in robot_map.into_iter() {
            for q in QUADRANTS.clone().into_iter() {
                let bounds = all_bounds.get(&q).unwrap();
                if in_bounds(&loc, &bounds) {
                    result.entry(q)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            }
        }
        println!("{:#?}", result);
        result
    }

    pub fn step_all_robots(&mut self) {
        for i in 0..self.robots.len() {
            self._step_robot_at(i);
        }
    }

    pub fn _step_robot_at(&mut self, robot_idx: usize) {
        let robot = &self.robots[robot_idx];
        let next_loc = self.get_next_loc(robot);
        
        let robot = &mut self.robots[robot_idx];
        robot.loc.x = next_loc.x;
        robot.loc.y = next_loc.y;
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    pub fn print_quads(&self) {
        let robot_map = self._get_loc_robot_counts();
        let all_bounds = self.get_quadrant_bounds();
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let l = Location { x, y };
                let mut found = false;
                let mut in_quad = false;
                for q in QUADRANTS.clone().into_iter() {
                    if in_bounds(&l, all_bounds.get(&q).unwrap()) {
                        in_quad = true;
                        if robot_map.contains_key(&l) {
                            found = true;
                            print!("{}", robot_map.get(&l).unwrap());
                        }
                    }
                }
                if !in_quad {
                    print!(" ");
                } else if !found {
                    print!(".");
                }
            }
            println!("");
        }

    }

    pub fn print(&self) {
        let robot_map = self._get_loc_robot_counts();
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let l = Location { x, y };
                if robot_map.contains_key(&l) {
                    print!("{}", robot_map.get(&l).unwrap());
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    pub fn compute_connectivity(&self) -> f64 {
        let robot_map = self._get_loc_robot_counts();
        let mut robots_with_neighbors = 0;
        for r in self.robots.iter() {
            let mut neighbors = 0;
            for x_offset in -2..2 {
                for y_offset in -2..2 {
                    if !(x_offset == 0 && y_offset == 0) {
                        let l = Location { x: r.loc.x + x_offset, y: r.loc.y + y_offset };
                        if robot_map.contains_key(&l) {
                            neighbors += 1;
                        }
                    }
                }
            }
            if neighbors >= 2 {
                robots_with_neighbors += 1;
            }
        }
        (robots_with_neighbors as f64) / (self.robots.len() as f64)
    }
}

pub fn parse_input(input: &String) -> Vec<Robot> {
    let mut robots = vec![];
    let rx = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").expect("invalid regex");

    for line in input.lines() {
        //println!("LINE: {}", line);
        for (_, [x_pos, y_pos, x_vel, y_vel]) in rx.captures_iter(&line).map(|c| c.extract()) {
            //println!("\tCAPTURE: {} {} {} {}", x_pos, y_pos, x_vel, y_vel);
            let x_pos = x_pos.parse::<isize>().unwrap();
            let y_pos = y_pos.parse::<isize>().unwrap();

            let x_vel = x_vel.parse::<isize>().unwrap();
            let y_vel = y_vel.parse::<isize>().unwrap();

            robots.push(Robot {
                loc: Location {
                    x: x_pos,
                    y: y_pos,
                },
                vel: Velocity {
                    x: x_vel,
                    y: y_vel,
                }
            });
        }
    }
    robots
}

pub fn solution1(path: &PathBuf, map_dimensions: Dimensions) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input, map_dimensions)
}

pub fn _solution1(input: &String, map_dimensions: Dimensions) -> isize {
    let mut l = Lobby::new(map_dimensions);
    for robot in parse_input(&input).into_iter() {
        l.add_robot(robot);
    }
    //println!("###########################");
    //l.print();
    //println!("###########################");
    for _s in 0..100 {
        l.step_all_robots();
        //println!("###########################");
        //l.print();
        //println!("###########################");
    }
    println!("###########################");
    l.print();
    println!("###########################");
    let mut result = 1;
    let mut scores = l.get_quadrant_scores();
    println!("{:#?}", l.robots);
    for q in QUADRANTS.clone().into_iter() {
        let score = scores.entry(q).or_insert(1);
        result *= *score;
    }

    result
}

pub fn solution2(path: &PathBuf, map_dimensions: Dimensions) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input, map_dimensions)
}

pub fn _solution2(input: &String, map_dimensions: Dimensions) -> isize {
    let mut l = Lobby::new(map_dimensions);
    for robot in parse_input(&input).into_iter() {
        l.add_robot(robot);
    }
    println!("########## SECOND 0 #################");
    //l.print();
    println!("###########################");
    for _s in 0..1000000000 {
        l.step_all_robots();
        if l.compute_connectivity() > 0.6 {
            println!("########### SECOND {} ################", _s + 1);
            l.print();
            println!("###########################");
        }
        if _s % 10_000 == 0 {
            println!(".. {} ..", _s);
        }
        //println!("###########################");
        //l.print();
        //println!("###########################");
    }
    0
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn step_all_single() {
        let mut l = Lobby::new(Dimensions { x: 3, y: 3 });
        let r = Robot {
            loc: Location { x: 0, y: 0 },
            vel: Velocity { x: 1, y: 1 },
        };
        l.add_robot(r);
        {
            l.step_all_robots();
            let r = &l.robots[0];
            assert_eq!(r.loc.x, 1, "advance x 0");
            assert_eq!(r.loc.y, 1, "advance y 0");
        }

        {
            l.step_all_robots();
            let r = &l.robots[0];
            assert_eq!(r.loc.x, 2, "advance x 1");
            assert_eq!(r.loc.y, 2, "advance y 1");
        }

        {
            l.step_all_robots();
            let r = &l.robots[0];
            assert_eq!(r.loc.x, 0, "advance x 2");
            assert_eq!(r.loc.y, 0, "advance y 2");
        }

        {
            l.step_all_robots();
            let r = &l.robots[0];
            assert_eq!(r.loc.x, 1, "advance x 3");
            assert_eq!(r.loc.y, 1, "advance y 3");
        }
    }

    #[test]
    fn get_quadrant_bounds() {
        let l = Lobby::new(Dimensions { x: 3, y: 3 });
        let bounds = l.get_quadrant_bounds();

        assert_eq!(
            bounds.get(&Quadrant::One).unwrap(),
            &BoundingBox {
                upper_left: Location { x: 0, y: 0 },
                lower_right: Location { x: 0, y: 0 },
            }
        );

        assert_eq!(
            bounds.get(&Quadrant::Two).unwrap(),
            &BoundingBox {
                upper_left: Location { x: 0, y: 2 },
                lower_right: Location { x: 0, y: 2 },
            }
        );

        assert_eq!(
            bounds.get(&Quadrant::Three).unwrap(),
            &BoundingBox {
                upper_left: Location { x: 2, y: 0 },
                lower_right: Location { x: 2, y: 0 },
            }
        );

        assert_eq!(
            bounds.get(&Quadrant::Four).unwrap(),
            &BoundingBox {
                upper_left: Location { x: 2, y: 2 },
                lower_right: Location { x: 2, y: 2 },
            }
        );
    }

    //#[test]
    //fn example_day_14_1_single() {
    //    let path = common::get_test_data_path("day14/case2.txt").unwrap();
    //    let result = solution1(&path, Dimensions { x: 11, y: 7 });
    //    assert_eq!(result, 1, "14_1_single example");
    //}

    #[test]
    fn example_day_14_1() {
        let path = common::get_test_data_path("day14/case1.txt").unwrap();
        let result = solution1(&path, Dimensions { x: 11, y: 7 });
        assert_eq!(result, 12, "14_1 example");
    }
}
