use std::fs;
use std::path::PathBuf;
use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub mod constants {
    pub const INPUT_PATH: &str = "day6/input.txt";
}

type Pos = (usize, usize);
type IPos = (isize, isize);


enum Occupant {
    Guard(Guard),
    Obstacle(Obstacle),
}

enum Orientation {
    Up,
    Down,
    Left,
    Right
}

struct Guard {
    orientation: Orientation,
}

impl Guard {
    fn turn_right(&mut self) {
        let new_orientation = match self.orientation {
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
        };
        self.orientation = new_orientation;
    }

    fn symbol(&self) -> char {
        match self.orientation {
            Orientation::Up => '^',
            Orientation::Down => 'v',
            Orientation::Left => '<',
            Orientation::Right => '>',
        }
    }
}

struct Obstacle {
}

impl Obstacle {
    fn symbol(&self) -> char {
        '#'
    }
}

struct Tile {
    occupant: Option<Occupant>,
    visited: bool,
}

impl Tile {

    fn place(&mut self, occupant: Occupant) {
        match self.occupant {
            None => {
                self.occupant = Some(occupant);
            },
            _ => panic!("attempting to place on occupied tile"),
        }
    }

    fn take(&mut self) -> Occupant {
        let o = self.occupant.take();
        match o {
            Some(occupant) => {
                self.occupant = None;
                occupant
            },
            _ => panic!("attempting to take but no occupant found"),
        }
    }
}

struct LabMap {
    tiles: HashMap<Pos, Tile>,
    num_rows: usize,
    num_cols: usize,
    guard_pos: Option<Pos>,
}

impl LabMap {

    fn new(num_cols: usize, num_rows: usize) -> Self {
        Self {
            tiles: HashMap::new(),
            num_rows,
            num_cols,
            guard_pos: None,
        }        
    }

    fn place_tile(&mut self, pos: Pos) {
        let x_pos = pos.0;
        let y_pos = pos.1;

        assert!(
            x_pos < self.num_cols,
            "tile pos ({}, {}) out of expected x bounds [0, {}]",
            x_pos,
            y_pos,
            self.num_cols - 1,
        );
        assert!(
            y_pos < self.num_rows,
            "tile pos ({}, {}) out of expected y bounds [0, {}]",
            x_pos,
            y_pos,
            self.num_rows - 1,
        );

        match self.tiles.entry(pos) {
            Entry::Vacant(entry) => entry.insert(Tile { occupant: None, visited: false }),
            Entry::Occupied(_entry) => panic!("attempting to place tile where another tile exists ({}, {})", x_pos, y_pos),
        };
    }

    fn place(&mut self, pos: Pos, occupant: Occupant) {
        let tile = self.tiles.get_mut(&pos).unwrap();
        tile.place(occupant);
    }

    fn place_guard(&mut self, pos: Pos, guard: Guard) {
        self.place(pos, Occupant::Guard(guard));
        let tile = self.tiles.get_mut(&pos).unwrap();
        tile.visited = true;
        self.guard_pos = Some(pos);
    }

    fn place_obstacle(&mut self, pos: Pos, obstacle: Obstacle) {
        self.place(pos, Occupant::Obstacle(obstacle))
    }

    fn take_guard(&mut self, pos: Pos) -> Guard {
        let tile = self.tiles.get_mut(&pos).unwrap();
        self.guard_pos = None;
        match tile.take() {
            Occupant::Guard(guard) => guard,
            _ => panic!("guard not occupying tile for take"),
        }
    }

    fn take_obstacle(&mut self, pos: Pos) -> Obstacle {
        let tile = self.tiles.get_mut(&pos).unwrap();
         match tile.take() {
            Occupant::Obstacle(obs) => obs,
            _ => panic!("guard not occupying tile for take"),
        }
    }

    fn in_bounds_at(&self, ipos: IPos) -> bool {
        let x_pos = ipos.0;
        let y_pos = ipos.1;

        let (num_cols, num_rows) = to_ipos((self.num_cols, self.num_rows));

        let x_in_bounds = x_pos >= 0 && x_pos < num_cols;
        let y_in_bounds = y_pos >= 0 && y_pos < num_rows;
        
        x_in_bounds && y_in_bounds
    }

    fn vacant_at(&self, pos: Pos) -> bool {
        match self.tiles.get(&pos).unwrap().occupant {
            None => true,
            _ => false,
        }
    }

    fn from_str(input: &String) -> Self {
        let num_cols = input.lines().count();
        let num_rows = input.lines().nth(0).unwrap().len();

        let mut map = Self::new(num_rows, num_cols);
        
        for (row_idx, row) in input.lines().enumerate() {

            for (col_idx, byte) in row.as_bytes().into_iter().enumerate() {
                let pos = (col_idx, row_idx);
                map.place_tile(pos);

                match char::try_from(*byte).unwrap() {
                    '.' => (),
                    '#' => {
                        map.place_obstacle(pos, Obstacle {});
                    },
                    '^' => {
                        map.guard_pos = Some(pos);
                        map.place_guard(pos, Guard { orientation: Orientation::Up });
                    },
                    c => panic!("encountered unknown char: {}", c),
                };
            }
        }
        map
    }

    fn count_tiles(&self, filter: Box<dyn Fn(&Tile) -> bool>) -> usize {
        let mut count = 0;
        for y in 0..self.num_rows {
            for x in 0..self.num_cols {
                let tile = self.tiles.get(&(x, y)).unwrap();
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
        for y in 0..self.num_rows {
            for x in 0..self.num_cols {
                let tile = self.tiles.get(&(x, y)).unwrap();
                let c = match &tile.occupant {
                    None => match tile.visited {
                        true => 'X',
                        false => '.',
                    },
                    Some(Occupant::Guard(guard)) => guard.symbol(),
                    Some(Occupant::Obstacle(obs)) => obs.symbol(),
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

fn to_ipos(pos: Pos) -> IPos {
    let x = isize::try_from(pos.0).unwrap();
    let y = isize::try_from(pos.1).unwrap();
    (x, y)
}

fn to_pos(ipos: IPos) -> Pos {
    let x = usize::try_from(ipos.0).unwrap();
    let y = usize::try_from(ipos.1).unwrap();
    (x, y)
}

fn get_forward_step_pos(guard_pos: Pos, guard: &Guard) -> (isize, isize) {
    let ipos = to_ipos(guard_pos);
    let (offset_x, offset_y) = match guard.orientation {
        Orientation::Left => (-1, 0),
        Orientation::Right => (1, 0),
        Orientation::Up => (0, -1),
        Orientation::Down => (0, 1),
    };
    (ipos.0 + offset_x, ipos.1 + offset_y)
}

pub fn _solution1(input: &String) -> usize {
    let mut map = LabMap::from_str(&input);
    'walk_path: loop {
        let guard_pos = map.guard_pos.clone().unwrap();
        let mut guard = map.take_guard(guard_pos);

        'next_step: loop {
            let next_ipos = get_forward_step_pos(guard_pos, &guard);
            if map.in_bounds_at(next_ipos) {
                let next_pos = to_pos(next_ipos);
                if map.vacant_at(next_pos) {
                    map.place_guard(next_pos, guard);
                    println!("{}", map);
                    break 'next_step;
                } else {
                    guard.turn_right();
                }
            } else {
                // guard walked off the map
                break 'walk_path;
            }
        };
    };
    map.count_tiles(Box::new(|tile| {
        tile.visited
    }))

}

//pub fn solution1(path: &PathBuf) -> usize {
//    let input =  fs::read_to_string(path).unwrap();
//    _solution1(&input)
//}
//
//pub fn _solution1(input: &String) -> usize {
//    let mut map = LabMap::from_str(&input);
//    loop {
//        match map.move_guard() {
//            Movement::OffMap => break,
//            _ => (),
//        }
//    };
//    map.count_tiles(Box::new(|tile| {
//        match tile {
//            Tile::Empty(visited) => *visited,
//            _ => false,
//        }
//    }))
//}

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

    //#[test]
    //fn test_example_day2() {
    //    assert!(false, "todo")
    //}
}
