use std::fs;
use std::path::PathBuf;
use std::fmt;

pub mod constants {
    pub const INPUT_PATH: &str = "day6/input.txt";
}

enum Movement {
    Up,
    Down,
    Left,
    Right,
    OffMap,
}

enum GuardOrientation {
    Up,
    Down,
    Left,
    Right
}

enum Tile {
    Empty(bool), // visited or not
    Obstacle,
    Guard(GuardOrientation),
}

struct LabMap {
    grid: Vec<Vec<Tile>>,
    guard_pos: (usize, usize),
}

impl LabMap {
    fn from_str(input: &String) -> Self {
        let mut grid = Vec::new();
        let mut guard_pos = (usize::MAX, usize::MAX);

        for line in input.lines() {
            let mut row = Vec::new();
            for byte in line.as_bytes() {
                let tile = match char::try_from(*byte).unwrap() {
                    '.' => Tile::Empty(false),
                    '#' => Tile::Obstacle,
                    '^' => {
                        guard_pos = (row.len(), grid.len());
                        Tile::Guard(GuardOrientation::Up)
                    },
                    c => panic!("encountered unknown char: {}", c),
                };
                row.push(tile);
            }
            grid.push(row);
        }

        assert_ne!(guard_pos, (usize::MAX, usize::MAX), "did not init guard position");

        Self {
            grid,
            guard_pos,
        }
    }

    fn get_num_rows(&self) -> usize {
        self.grid.len()
    }

    fn get_num_cols(&self) -> usize {
        self.grid[0].len()
    }

    fn get_next_guard_orientation(&self, orientation: GuardOrientation) -> GuardOrientation {
        match orientation {
            GuardOrientation::Left => GuardOrientation::Up,
            GuardOrientation::Right => GuardOrientation::Down,
            GuardOrientation::Up => GuardOrientation::Right,
            GuardOrientation::Down => GuardOrientation::Left,
        }
    }

    fn move_guard(&mut self) -> Movement {
        let guard_pos = self.guard_pos;
        let guard = &self.grid[guard_pos.1][guard_pos.0];
        let (offset_x, offset_y, desired_movement, current_orientation) = match guard {
            Tile::Guard(orientation) => match orientation {
                GuardOrientation::Left => (-1, 0, Movement::Left, GuardOrientation::Left),
                GuardOrientation::Right => (1, 0, Movement::Right, GuardOrientation::Right),
                GuardOrientation::Up => (0, -1, Movement::Up, GuardOrientation::Up),
                GuardOrientation::Down => (0, 1, Movement::Down, GuardOrientation::Down),
            },
            _ => panic!("guard_pos not pointing to guard")
        };
        let new_x = isize::try_from(guard_pos.0).unwrap() + offset_x;
        let new_y = isize::try_from(guard_pos.1).unwrap() + offset_y;

        let out_of_bounds_x = new_x < 0 || new_x >= isize::try_from(self.get_num_cols()).unwrap();
        let out_of_bounds_y = new_y < 0 || new_y >= isize::try_from(self.get_num_rows()).unwrap();
        if out_of_bounds_x || out_of_bounds_y {
            self.grid[guard_pos.1][guard_pos.0] = Tile::Empty(true);
            self.guard_pos = (usize::MAX, usize::MAX);
            return Movement::OffMap;
        }

        let new_x = usize::try_from(new_x).unwrap();
        let new_y = usize::try_from(new_y).unwrap();
        let next_tile = &self.grid[new_y][new_x];

        match next_tile {
            Tile::Empty(_visited) => {
                self.grid[guard_pos.1][guard_pos.0] = Tile::Empty(true);
                self.grid[new_y][new_x] = Tile::Guard(current_orientation);
                self.guard_pos = (new_x, new_y);
                desired_movement
            },
            Tile::Obstacle => {
                // re-orient the guard and try to move again
                let next_orientation = self.get_next_guard_orientation(current_orientation);
                self.grid[guard_pos.1][guard_pos.0] = Tile::Guard(next_orientation);
                self.move_guard()
            },
            Tile::Guard(_) => panic!("guard should not be moving into another guard"),
        }
    }

    fn count_tiles(&self, filter: Box<dyn Fn(&Tile) -> bool>) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            for tile in row.iter() {
                if filter(tile) {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for LabMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid.iter() {
            for tile in row.iter() {
                let c = match tile {
                    Tile::Empty(true) => 'X', // visited
                    Tile::Empty(false) => '.', // not visited
                    Tile::Obstacle => '#',
                    Tile::Guard(orientation) => {
                        match orientation {
                            GuardOrientation::Up => '^',
                            GuardOrientation::Down => 'V',
                            GuardOrientation::Right => '>',
                            GuardOrientation::Left => '<',
                        }
                    },
                };
                write!(f, "{}", c).expect("error writing");
            }
            writeln!(f, "").expect("error writing");
        }
        write!(f, "")
    }
}

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> usize {
    let mut map = LabMap::from_str(&input);
    loop {
        match map.move_guard() {
            Movement::OffMap => break,
            _ => (),
        }
    };
    map.count_tiles(Box::new(|tile| {
        match tile {
            Tile::Empty(visited) => *visited,
            _ => false,
        }
    }))
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
    fn test_example_day1() {
        //let path = fs::read_to_string(common::get_test_data_path("day6/case1.txt").unwrap()).unwrap();
        ////let result = solution1(&path);
        //let mut map = LabMap::from_str(&path);
        //println!("{}", map);
        //map.move_guard();
        let path = common::get_test_data_path("day6/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 41, "counts guard path correctly")
    }

    #[test]
    fn test_example_day2() {
        assert!(false, "todo")
    }
}
