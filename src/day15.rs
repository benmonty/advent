use core::panic;
use std::fs;
use std::path::PathBuf;
use std::fmt;

use rustc_hash::FxHashMap;

pub mod constants {
    pub const INPUT_PATH: &str = "day15/input.txt";

    pub const CHAR_LEFT: char = '<';
    pub const CHAR_RIGHT: char = '>';
    pub const CHAR_UP: char = '^';
    pub const CHAR_DOWN: char = 'v';

    pub const CHAR_ROBOT: char = '@';
    pub const CHAR_BOX: char = 'O';
    pub const CHAR_WALL: char = '#';
    pub const CHAR_EMPTY: char = '.';

}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Loc {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct Dimensions {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub enum Entity {
    Robot,
    Wall,
    Box,
}

#[derive(Debug)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Warehouse {
    robot_loc: Loc,
    map: FxHashMap<Loc, Option<Entity>>,
    dimensions: Dimensions,
}

struct PushErr;

pub fn parse_movements(input: &String) -> Vec<Movement> {
    let mut mv_lines = input.lines().skip_while(|l| !l.is_empty());
    mv_lines.next();

    let mut movements = vec![];

    for line in mv_lines {
        for c in line.chars() {
            match c {
                constants::CHAR_LEFT => movements.push(Movement::Left),
                constants::CHAR_RIGHT => movements.push(Movement::Right),
                constants::CHAR_UP => movements.push(Movement::Up),
                constants::CHAR_DOWN => movements.push(Movement::Down),
                c => panic!("unknown movement char detected: {}", c),
            }
        }
    }

    movements
}

pub fn _loc_u(x: usize, y: usize) -> Loc {
    Loc {
        x: isize::try_from(x).unwrap(),
        y: isize::try_from(y).unwrap(),
    }
}
pub fn _loc_i(x: isize, y: isize) -> Loc {
    Loc { x, y }
}

impl Warehouse {
    pub fn from(input: &String) -> Self {
        let map_lines = input.lines().take_while(|l| !l.is_empty());
        let y_tiles = map_lines.clone().count();
        let x_tiles = map_lines.clone().next().unwrap().len();
        let mut wh = Self {
            robot_loc: _loc_i(isize::MAX, isize::MAX),
            map: FxHashMap::default(),
            dimensions: Dimensions {
                x: isize::try_from(x_tiles).unwrap(),
                y: isize::try_from(y_tiles).unwrap(),
            },
        };

        for (y, line) in map_lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                let loc = _loc_u(x, y);
                match c {
                    constants::CHAR_ROBOT => {
                        wh.insert(&loc, Some(Entity::Robot));
                    }
                    constants::CHAR_WALL => {
                        wh.insert(&loc, Some(Entity::Wall));
                    }
                    constants::CHAR_BOX => {
                        wh.insert(&loc, Some(Entity::Box));
                    }
                    constants::CHAR_EMPTY => {
                        wh.insert(&loc, None);
                    }
                    _ => panic!("unknown char found while parsing map"),
                }
            }
        }

        assert_ne!(wh.robot_loc, _loc_i(isize::MAX, isize::MAX), "robot loc not updated");

        wh
    }

    pub fn _get_push_dest(&self, src: &Loc, dir: &Movement) -> Loc {
        match dir {
            Movement::Up => {
                Loc {
                    x: src.x,
                    y: src.y - 1,
                }
            },
            Movement::Down => {
                Loc {
                    x: src.x,
                    y: src.y + 1,
                }
            },
            Movement::Left => {
                Loc {
                    x: src.x - 1,
                    y: src.y,
                }
            },
            Movement::Right => {
                Loc {
                    x: src.x + 1,
                    y: src.y,
                }
            },
        }
    }

    pub fn _try_push(&mut self, src: &Loc, dir: &Movement) -> Result<Loc, PushErr> {
        let dest = self._get_push_dest(src, dir);
        match self.map.get(&dest).unwrap() {
            None => {
                self._mv(&src, &dest);
                Ok(dest)
            },
            Some(Entity::Box) | Some(Entity::Robot) => {
                match self._try_push(&dest, &dir) {
                    Ok(_) => {
                        self._mv(&src, &dest);
                        Ok(dest)
                    },
                    e => e,
                }
            },
            Some(Entity::Wall) => {
                Err(PushErr)
            },
        }
    }

    pub fn _mv(&mut self, src: &Loc, dest: &Loc) {
        println!("######");
        println!("SRC: {:#?}", src);
        println!("DEST: {:#?}", dest);
        println!("######");
        match self.map.get(&dest).unwrap() {
            None => {
                match self.map.remove(&src).unwrap() {
                    Some(entity) => {
                        self.map.entry(src.clone()).or_insert(None);
                        self.map.entry(dest.clone()).and_modify(|v| *v = Some(entity));
                    },
                    None => panic!("attempting to move None"),
                }
            },
            Some(_) => panic!("attempting to move to an occupied loc"),
        }
    }

    pub fn try_push_robot(&mut self, dir: &Movement) -> Result<Loc, PushErr> {
        match self._try_push(&self.robot_loc.clone(), &dir) {
            Ok(loc) => {
                self.robot_loc = loc;
                Ok(self.robot_loc.clone())
            },
            e => e,
        }
    }

    pub fn insert(&mut self, loc: &Loc, entity: Option<Entity>) {
        match &entity {
            Some(Entity::Robot) => {
                self.robot_loc = loc.clone();
                self.map.entry(loc.clone()).or_insert(entity);
            },
            Some(_) => {
                self.map.entry(loc.clone()).or_insert(entity);
            }
            None => {
                self.map.entry(loc.clone()).or_insert(None);
            },
        }
    }

    pub fn compute_gps_sum(&self) -> isize {
        let mut sum = 0;
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                match self.map.get(&Loc { x, y }).unwrap() {
                    Some(Entity::Box) => {
                        sum += 100 * y + x
                    },
                    _ => (),
                }
            }
        }
        sum
    }
}

impl fmt::Display for Warehouse {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                match self.map.get(&_loc_i(x, y)) {
                    Some(opt_e) => {
                        match opt_e {
                            Some(Entity::Robot) => write!(f, "{}", constants::CHAR_ROBOT),
                            Some(Entity::Box) => write!(f, "{}", constants::CHAR_BOX),
                            Some(Entity::Wall) => write!(f, "{}", constants::CHAR_WALL),
                            None => write!(f, "{}", constants::CHAR_EMPTY)
                        }
                    },
                    _ => panic!("loc is missing data"),
                }.expect("failed to write");
            }
            write!(f, "\n").expect("failed to write");
        }
        write!(f, "\n")
    }

}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> isize {
    let mut wh = Warehouse::from(&input);
    let moves = parse_movements(&input);

    print!("{}", wh);
    for mv in moves {
        let _ = wh.try_push_robot(&mv);
        print!("{}", wh);
    }
    wh.compute_gps_sum()
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(_input: &String) -> usize {
    0
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example_day_15_1_1() {
        let path = common::get_test_data_path("day15/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_example_day_15_1_2() {
        let path = common::get_test_data_path("day15/case2.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_example_day2() {
        assert!(false, "todo")
    }
}
