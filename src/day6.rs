use std::fs;
use std::path::PathBuf;
use std::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;

pub mod constants {
    pub const INPUT_PATH: &str = "day6/input.txt";
}

type Pos = (usize, usize);
type IPos = (isize, isize);

enum Occupant {
    Guard(Guard),
    Obstacle(Obstacle),
    CustomObstacle(CustomObstacle),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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

struct CustomObstacle {
}

impl CustomObstacle {
    fn symbol(&self) -> char {
        'O'
    }
}

struct Tile {
    occupant: Option<Occupant>,
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
    visits: HashSet<Pos>,
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
            visits: HashSet::new(),
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
            Entry::Vacant(entry) => entry.insert(Tile { occupant: None }),
            Entry::Occupied(_entry) => panic!("attempting to place tile where another tile exists ({}, {})", x_pos, y_pos),
        };
    }

    fn place(&mut self, pos: Pos, occupant: Occupant) {
        let tile = self.tiles.get_mut(&pos).unwrap();
        tile.place(occupant);
    }

    fn place_guard(&mut self, pos: Pos, guard: Guard) {
        self.place(pos, Occupant::Guard(guard));
        self.visits.insert(pos);
        self.guard_pos = Some(pos);
    }

    fn place_obstacle(&mut self, pos: Pos, obstacle: Obstacle) {
        self.place(pos, Occupant::Obstacle(obstacle))
    }

    fn place_custom_obstacle(&mut self, pos: Pos, obstacle: CustomObstacle) {
        self.place(pos, Occupant::CustomObstacle(obstacle))
    }

    fn take_guard(&mut self, pos: Pos) -> Guard {
        let tile = self.tiles.get_mut(&pos).unwrap();
        self.guard_pos = None;
        match tile.take() {
            Occupant::Guard(guard) => guard,
            _ => panic!("guard not occupying tile for take"),
        }
    }

    fn clear_guard(&mut self) {
        if let Some(pos) = self.guard_pos {
            self.take_guard(pos);
        }
    }

    fn take_obstacle(&mut self, pos: Pos) -> Obstacle {
        let tile = self.tiles.get_mut(&pos).unwrap();
         match tile.take() {
            Occupant::Obstacle(obs) => obs,
            _ => panic!("obstacle not occupying tile for take"),
        }
    }

    fn take_custom_obstacle(&mut self, pos: Pos) -> CustomObstacle {
        let tile = self.tiles.get_mut(&pos).unwrap();
         match tile.take() {
            Occupant::CustomObstacle(obs) => obs,
            _ => panic!("custom obstacle not occupying tile for take"),
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

    fn clear_visits(&mut self) {
        self.visits = HashSet::new();
    }

    fn sync_state_history(&mut self, history: &Vec<GuardState>) {
        self.clear_visits();
        self.clear_guard();
        for state in history.iter() {
            self.visits.insert(state.pos);
        }
        let last_state = history.last().unwrap();
        self.place_guard(last_state.pos, Guard { orientation: last_state.orientation });
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
                let pos = (x, y);
                let tile = self.tiles.get(&pos).unwrap();
                let c = match &tile.occupant {
                    None => match self.visits.contains(&pos) {
                        true => 'X',
                        false => '.',
                    },
                    Some(Occupant::Guard(guard)) => guard.symbol(),
                    Some(Occupant::Obstacle(obs)) => obs.symbol(),
                    Some(Occupant::CustomObstacle(obs)) => obs.symbol(),
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
    map.visits.len()
}

//pub fn solution1(path: &PathBuf) -> usize {
//    let input =  fs::read_to_string(path).unwrap();
//    _solution1(&input)
//}
//
//pub fn _solution1(input: &String) -> usize {
//    let mut map = LabMap::from_str(&input);
//    loop {
//        match map.move_guard() {k
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

struct GuardState {
    pos: Pos,
    orientation: Orientation,
}

fn resume_walk(map: &mut LabMap, history: &Vec<GuardState>) -> Option<VecDeque<GuardState>> {
    //println!("RESUME");
    //println!("history({})", history.len());
    map.sync_state_history(history);
    //println!("{}", map);
    let mut rest_of_walk = VecDeque::new();
    let mut visits: HashSet<(Pos, Orientation)> = HashSet::new();
    //let start_pos = history.last().unwrap().pos;
    //let start_orientation = history.last().unwrap().orientation;
    let mut overlaps = 0;
    'walk_path: loop {
        let guard_pos = map.guard_pos.unwrap();
        let mut guard = map.take_guard(guard_pos);

        'next_step: loop {
            let next_ipos = get_forward_step_pos(guard_pos, &guard);
            if map.in_bounds_at(next_ipos) {
                let next_pos = to_pos(next_ipos);
                if map.vacant_at(next_pos) {
                    rest_of_walk.push_back(GuardState{
                        pos: next_pos,
                        orientation: guard.orientation,
                    });
                    if visits.contains(&(next_pos, guard.orientation)) {
                        //println!("CYCLE");
                        //println!("{}", map);
                        return None; // cycle detected
                    } else {
                        visits.insert((next_pos, guard.orientation));
                    }
                    map.place_guard(next_pos, guard);
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
    //println!("NOCYCLE");
    //println!("{}", map);
    Some(rest_of_walk)
}

pub fn _solution2(input: &String) -> usize {
    let mut map = LabMap::from_str(&input);
    let guard_pos = map.guard_pos.clone().unwrap();
    let guard = map.take_guard(guard_pos);
    let mut history = vec![GuardState { pos: guard_pos, orientation: guard.orientation }];
    let mut rest_of_walk = resume_walk(&mut map, &history).unwrap();
    let mut num_cycles = 0;
    let mut invalid_placements = HashSet::new();
    invalid_placements.insert(history[0].pos);
    while rest_of_walk.len() != 0 {
        let obs_pos = rest_of_walk[0].pos;
        if invalid_placements.contains(&obs_pos) {
            history.push(rest_of_walk.pop_front().unwrap());
            continue;
        }
        map.place_custom_obstacle(obs_pos, CustomObstacle {});
        match resume_walk(&mut map, &history) {
            None => num_cycles += 1,
            _ => (), // completed, throw away the walk
        }
        map.take_custom_obstacle(obs_pos);
        invalid_placements.insert(obs_pos);
        history.push(rest_of_walk.pop_front().unwrap());
    }
    num_cycles
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
        let path = common::get_test_data_path("day6/case1.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 6, "counts guard path correctly")
    }
}
