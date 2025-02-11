use std::{collections::VecDeque, fs};
use std::path::PathBuf;
use rustc_hash::{FxHashMap, FxHashSet};


// Node
//   neighbors: HashMap<Dir, Loc>

// Nodes: HashMap<Loc, Node>
// Reindeer Loc
// Reindeer Dir

// Path
//   Reindeer
//   visits: HashSet<Loc>
//   history: Vec<Action>
//   cost: usize
//   dest: Loc

// get_min_cost(path)
//
//      _get_min_cost(nodes, visits, dest, current_loc, current_dir, current_cost, best_cost)
//          at dest? return 0
//          at visited loc? return usize::max/2
//          cost greater than best? return usize::max/2
//          visits.add(current_loc)
//          can move straight?
//              Option<best_cost> = 1 + _get_min_cost(nodes, visits, dest, straight_loc,
//              current_dir, usize::max)
//          can move left?
//              Option<best_cost> = 1001 + _get_min_cost(nodes, visits, dest, turn_left_loc(),
//              turn_left_dir, best_cost)
//              move left
//          can move right?
//              Option<best_cost> = 1001 + _get_min_cost(nodes, visits, dest, turn_right_loc(),
//              turn_right_dir, best_cost)
//              move right
//              
/*

    _get_best_path(nodes, visits, visits_map, dest, loc, dir, current_cost, best_cost)
        at dest? return 0 
        at visited loc? return usize::max/2
        cost greater than best? return usize::max/2
        visits.add(loc)
        can move straight?
            best_cost = 1 + _get_min_cost(nodes, visits, dest, straight_loc,



    vec![Turn(dir)|GoForward(loc)] + _get_best_path(..)
    visits: HashSet<loc>
    total_cost
*/

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Loc {
    x: isize,
    y: isize,
}

impl Loc {
    pub fn from_u(x: usize, y: usize) -> Self {
        Self {
            x: isize::try_from(x).unwrap(),
            y: isize::try_from(y).unwrap(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Deer {
    loc: Loc,
    dir: Direction,
}

impl Deer {
    pub fn turn_left(&self) -> Self {
        let next_dir = match self.dir {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        };

        Self {
            loc: self.loc.clone(),
            dir: next_dir,
        }
    }

    pub fn turn_right(&self) -> Self {
        let next_dir = match self.dir {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        };

        Self {
            loc: self.loc.clone(),
            dir: next_dir,
        }
    }
}

pub enum Tile {
    Deer,
    Empty,
    Wall,
    Goal,
}

type BaseMap = FxHashMap<Loc, Tile>;

type NeighborMap = FxHashMap<Loc, FxHashMap<Direction, Loc>>;

pub struct Maze {
    base_map: BaseMap,
    deer_start: Deer,
    goal: Loc,
    neighbors: NeighborMap,
    dim_x: isize,
    dim_y: isize,
}

pub fn potential_neighbors(loc: &Loc) -> Vec<(Loc, Direction)> {
    vec![
        (Loc { x: loc.x + 1, y: loc.y }, Direction::East),
        (Loc { x: loc.x - 1, y: loc.y }, Direction::West),
        (Loc { x: loc.x, y: loc.y + 1 }, Direction::South),
        (Loc { x: loc.x, y: loc.y - 1 }, Direction::North),
    ]
}

#[derive(Clone, Debug)]
pub enum Rotation {
    Left90,
    Right90,
}

#[derive(Clone, Debug)]
pub enum Action {
    Move(Deer, Deer),
    Turn(Rotation),
}

#[derive(Debug)]
pub struct Move {
    actions: Vec<Action>,
    src_deer: Deer,
    dest_deer: Deer,
    cost: isize,
}


impl Maze {

    fn _parse_base(input: &String) -> BaseMap {
        let mut base_map = BaseMap::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => base_map.entry(Loc::from_u(x, y)).or_insert(Tile::Wall),
                    '.' => base_map.entry(Loc::from_u(x, y)).or_insert(Tile::Empty),
                    'S' => base_map.entry(Loc::from_u(x, y)).or_insert(Tile::Deer),
                    'E' => base_map.entry(Loc::from_u(x, y)).or_insert(Tile::Goal),
                    c => panic!("unexpected character encountered: {}", c),
                };
            }
        }
        base_map
    }

    pub fn _init_neighbors(loc: &Loc, base_map: &BaseMap, neighbor_map: &mut NeighborMap) {
        for pn in potential_neighbors(loc) {
            match base_map.get(&pn.0) {
                Some(Tile::Wall) => (),
                Some(Tile::Empty) | Some(Tile::Deer) | Some(Tile::Goal) => {
                    neighbor_map.entry(loc.clone()).or_insert(FxHashMap::default());
                    neighbor_map.entry(loc.clone())
                        .and_modify(|m| {
                            m.entry(pn.1).or_insert(pn.0);
                        });
                },
                None => (),
            }
        }
    }

    pub fn from(input: &String) -> Self {

        let base_map = Maze::_parse_base(&input);
        let mut neighbor_map = NeighborMap::default();
        let mut deer: Option<Deer> = None;
        let mut goal: Option<Loc> = None;

        let dim_y = isize::try_from(input.lines().count()).unwrap();
        let dim_x = isize::try_from(input.lines().next().unwrap().chars().count()).unwrap();

        for (loc, tile) in base_map.iter() {
            match tile {
                Tile::Wall => (),
                Tile::Deer => {
                    deer = Some(Deer {
                        dir: Direction::East,
                        loc: loc.clone(),
                    });
                    Self::_init_neighbors(&loc, &base_map, &mut neighbor_map);
                }
                Tile::Goal => {
                    goal = Some(loc.clone());
                    Self::_init_neighbors(&loc, &base_map, &mut neighbor_map);
                }
                Tile::Empty => {
                    Self::_init_neighbors(&loc, &base_map, &mut neighbor_map);
                }
            }
        }

        Self {
            neighbors: neighbor_map,
            deer_start: deer.unwrap(),
            goal: goal.unwrap(),
            dim_x,
            dim_y,
            base_map,
        }
    }

    pub fn can_move(&self, src_deer: &Deer, r: Option<&Rotation>) -> Option<Move> {
        let mut dest_deer = src_deer.clone();
        let mut cost = constants::COST_MOVE;
        let mut actions: Vec<Action> = vec![];
        match r {
            Some(Rotation::Right90) => {
                dest_deer = src_deer.turn_right();
                cost += constants::COST_TURN;
                actions.push(Action::Turn(Rotation::Right90));
            },
            Some(Rotation::Left90) => {
                dest_deer = src_deer.turn_left();
                cost += constants::COST_TURN;
                actions.push(Action::Turn(Rotation::Left90));
            },
            None => (),
        }
        match self.neighbors.get(&src_deer.loc) {
            Some(neighbors) => {
                //println!("found neighbors");
                //println!("\t{:?}", dest_deer.dir);
                match neighbors.get(&dest_deer.dir) {
                    Some(loc) => {
                        //println!("\tfound dir neighbor");
                        dest_deer.loc = loc.clone();
                        actions.push(Action::Move(src_deer.clone(), dest_deer.clone()));
                        Some(Move {
                            src_deer: src_deer.clone(),
                            dest_deer,
                            cost,
                            actions,
                        })
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_valid_moves(&self, deer_src: &Deer) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let try_rotations = [
            None,
            Some(Rotation::Left90),
            Some(Rotation::Right90),
        ];

        for r in try_rotations.iter() {
            match self.can_move(deer_src, r.as_ref()) {
                Some(_move) => {
                    moves.push(_move);
                },
                None => (),
            }
        }
        moves
    }
}

#[derive(Clone)]
pub struct Solution {
    pub cost: isize,
    pub actions: Vec<Action>,
}
impl Solution {
    pub fn compute_cost(actions: &Vec<Action>) -> isize {
        actions.iter().map(|a| {
            match a {
                Action::Move(..) => {
                    1
                },
                Action::Turn(_) => {
                    1000
                },
            }
        }).sum()
    }

    pub fn new(actions: Vec<Action>) -> Self {
        let cost = Self::compute_cost(&actions);
        Self {
            actions,
            cost,
        }
    }
}

pub fn display(maze: &Maze, deer: &Deer, actions: &Vec<Action>) {
    let mut moves: FxHashMap<Loc, Direction> = FxHashMap::default();

    for a in actions {
        match a {
            Action::Move(sd, dd) => {
                moves.entry(sd.loc.clone()).or_insert(dd.dir.clone());
            },
            _ => (),
        }
    }

    println!("");

    let neighbors = maze.neighbors.get(&deer.loc).unwrap();
    println!("NEIGHBORS:");
    for (dir, loc) in neighbors.iter() {
        println!("\t{:?} {:?}", dir, loc);
    }

    for y in 0..maze.dim_y {
        for x in 0..maze.dim_x {
            let loc = Loc{ x, y };
            let symbol = if loc == maze.goal {
                'E'
            } else if loc == deer.loc {
                'S'
            } else if moves.contains_key(&loc) {
                match moves.get(&loc) {
                    Some(d) => {
                        match d {
                            Direction::North => '^',
                            Direction::South => 'v',
                            Direction::East => '>',
                            Direction::West => '>',
                        }
                    },
                    None => panic!("coord not found"),
                }
            } else {
                match maze.base_map.get(&loc) {
                    Some(Tile::Wall) => '#',
                    Some(Tile::Empty) => '.',
                    _ => panic!("should never get here"),
                }
            };
            print!("{}", symbol);
        }
        print!("\n");
    }



}

/*
pub fn _get_min_possible_remaining_cost(deer: &Deer, goal: &Loc) -> isize {
    let x_diff = isize::try_from(deer.loc.x.abs_diff(goal.x)).unwrap();
    let y_diff = isize::try_from(deer.loc.y.abs_diff(goal.y)).unwrap();
    let move_costs = x_diff + y_diff;

    let goal_x_dir = if goal.x > deer.loc.x {
        Some(Direction::East)
    } else if goal.x == deer.loc.x {
        None
    } else {
        Some(Direction::West)
    };
    let goal_y_dir = if goal.y < deer.loc.y {
        Some(Direction::North)
    } else if goal.y == deer.loc.y {
        None
    } else {
        Some(Direction::South)
    };

    let mut turn_costs = 0;

    match (&deer.dir, &goal_x_dir) {
        (Direction::East, Some(Direction::East)) => (),
        (Direction::South, Some(Direction::East)) => turn_costs += constants::COST_TURN,
        (Direction::North, Some(Direction::East)) => turn_costs += constants::COST_TURN,
        (Direction::West, Some(Direction::East)) => turn_costs += constants::COST_TURN,
        (Direction::East, Some(Direction::West)) => turn_costs += constants::COST_TURN,
        (Direction::South, Some(Direction::West)) => turn_costs += constants::COST_TURN,
        (Direction::North, Some(Direction::West)) => turn_costs += constants::COST_TURN,
        (Direction::West, Some(Direction::West)) => (),
        (_, None) => (),
        _ => panic!("should not occur"),
    }

    match (&deer.dir, &goal_y_dir) {
        (Direction::East, Some(Direction::North)) => turn_costs += constants::COST_TURN,
        (Direction::South, Some(Direction::North)) => turn_costs += constants::COST_TURN,
        (Direction::North, Some(Direction::North)) => (),
        (Direction::West, Some(Direction::North)) => turn_costs += constants::COST_TURN,
        (Direction::East, Some(Direction::South)) => turn_costs += constants::COST_TURN,
        (Direction::South, Some(Direction::South)) => (),
        (Direction::North, Some(Direction::South)) => turn_costs += constants::COST_TURN,
        (Direction::West, Some(Direction::South)) => turn_costs += constants::COST_TURN,
        (_, None) => (),
        _ => panic!("should not occur"),
    }

    move_costs + turn_costs
}
*/

pub fn _compute_best_costs(maze: &Maze, deer: &Deer) -> (FxHashMap<Deer, isize>, FxHashSet<Deer>) {
    let mut to_process: VecDeque<Deer> = VecDeque::from([deer.clone()]);
    let mut best_costs: FxHashMap<Deer, isize> = FxHashMap::default();
    //let mut best_paths: FxHashMap<Deer, Vec<Vec<Action>>> = FxHashMap::default();
    let mut end_deer: FxHashSet<Deer> = FxHashSet::default();

    best_costs.entry(deer.clone()).or_insert(0);
    //best_paths.entry(deer.clone()).or_insert(vec![vec![]]);
    
    while to_process.len() != 0 {
        //println!("{}", i);
        //println!("{}", to_process.len());
        let processing = to_process.pop_front().unwrap();
        if processing.loc == maze.goal {
            end_deer.insert(processing);
            continue;
        }
        for mv in maze.get_valid_moves(&processing).iter() {
            //println!("SRC: {:#?}", mv.src_deer);
            //println!("DEST: {:#?}", mv.dest_deer);
            let src_cost = best_costs.get(&processing).unwrap();
            let dest_cost = src_cost + mv.cost;
            match best_costs.get(&mv.dest_deer) {
                Some(existing_best) => {
                    if dest_cost < *existing_best {
                        if mv.dest_deer.loc == maze.goal {
                            println!("REPLACE NEW BEST: {}", dest_cost);
                        }
                        best_costs.entry(mv.dest_deer.clone()).and_modify(|v| *v = dest_cost);
                        //let best_src_paths = best_paths.get(&mv.src_deer).unwrap().clone();
                        //best_paths.entry(mv.dest_deer.clone()).and_modify(|v| {
                        //    *v = vec![];
                        //    for action_list in best_src_paths.into_iter() {
                        //        let mut al = action_list.clone();
                        //        al.append(&mut mv.actions.clone());
                        //        v.push(al);
                        //    }
                        //});
                        to_process.push_back(mv.dest_deer.clone());
                    } else if dest_cost == *existing_best {
                        if mv.dest_deer.loc == maze.goal {
                            println!("SAME NEW BEST: {}", dest_cost);
                        }
                        //let best_src_paths = best_paths.get(&mv.src_deer).unwrap().clone();
                        //best_paths.entry(mv.dest_deer.clone()).and_modify(|v| {
                        //    for action_list in best_src_paths.into_iter() {
                        //        let mut al = action_list.clone();
                        //        al.append(&mut mv.actions.clone());
                        //        v.push(al);
                        //    }
                        //});
                        //to_process.push_back(mv.dest_deer.clone());
                    } else {
                        // do nothing
                    }
                },
                None => {
                    //println!("NONE");
                    if mv.dest_deer.loc == maze.goal {
                        println!("NEW BEST: {}", dest_cost);
                    }
                    best_costs.entry(mv.dest_deer.clone()).or_insert(dest_cost);
                    //let best_src_paths = best_paths.get(&mv.src_deer).unwrap().clone();
                    //let mut dest_paths = vec![];
                    //for action_list in best_src_paths.into_iter() {
                    //    let mut al = action_list.clone();
                    //    al.append(&mut mv.actions.clone());
                    //    dest_paths.push(al);
                    //}
                    //best_paths.entry(mv.dest_deer.clone()).or_insert(dest_paths);
                    to_process.push_back(mv.dest_deer.clone());
                }
            }
        }
    }
    (best_costs, end_deer)
}

pub fn get_all_best_paths(maze: &Maze, deer: &Deer) -> Vec<Solution> {
    let (best_costs, all_end_deer) = _compute_best_costs(maze, deer);

    let backward_goal = deer.loc.clone();

    let mut all_solutions: Vec<Solution> = vec![];

    let best_solution_cost = get_best_goal_cost(&best_costs, &all_end_deer).unwrap();

    for d in all_end_deer.into_iter() {
        if *best_costs.get(&d).unwrap() > best_solution_cost {
            continue;
        }
        let backward_deer = d.turn_left().turn_left();
        let first_move = maze.can_move(&backward_deer, None).unwrap();
        let mut to_process: VecDeque<Solution> = VecDeque::from([Solution::new(first_move.actions.clone())]);

        while to_process.len() != 0 {
            let processing = to_process.pop_front().unwrap();

            let last_move_deer = match processing.actions.last().unwrap() {
                Action::Move(_src_deer, dest_deer) => {
                    dest_deer
                },
                _ => panic!("should always be a move"),
            };

            if last_move_deer.loc == backward_goal {
                println!("found backward goal");
                all_solutions.push(processing);
                continue;
            }

            for mv in maze.get_valid_moves(&last_move_deer).iter_mut() {
                let target_deer = vec![
                    mv.dest_deer.clone(),
                    mv.dest_deer.turn_left(),
                    mv.dest_deer.turn_left().turn_left(),
                    mv.dest_deer.turn_right(),
                ];
                let mut target_vals = vec![];
                for d in target_deer.iter() {
                    match best_costs.get(&d) {
                        Some(c) => {
                            target_vals.push(c);
                        }
                        _ => (),
                    }
                }

                if target_vals.iter().any(|v| *v + mv.cost + processing.cost <= best_solution_cost) {
                    println!("keep processing");
                    let mut sol = processing.clone();
                    sol.actions.append(&mut mv.actions);
                    sol.cost += mv.cost;
                    to_process.push_back(sol);
                } else {
                    // don't continue processing
                    println!("stop processing");
                }
            }
        }
    }
    println!("all solutions: {}", all_solutions.len());
    all_solutions
}

pub fn get_best_goal_cost(best_costs: &FxHashMap<Deer, isize>, end_deer: &FxHashSet<Deer>) -> Option<isize> {
    let mut min_cost = isize::MAX;
    for deer in end_deer.iter() {
        let best = best_costs.get(&deer).unwrap();
        if  *best < min_cost {
            min_cost = *best;
        }
    }
    Some(min_cost)
}


pub fn solve_for_min(maze: &Maze, deer: &Deer) -> Option<isize> {
    let (best_costs, end_deer) = _compute_best_costs(maze, deer);
    println!("neighbor map: {}", maze.neighbors.len());
    println!("best_costs: {}", best_costs.len());
    get_best_goal_cost(&best_costs, &end_deer)
}


pub mod constants {
    pub const INPUT_PATH: &str = "day16/input.txt";

    pub const COST_MOVE: isize = 1;
    pub const COST_TURN: isize = 1000;
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> isize {
    let maze = Maze::from(&input);
    let deer = maze.deer_start.clone();
    solve_for_min(&maze, &deer).unwrap()
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}


pub fn _solution2(input: &String) -> usize {
    let maze = Maze::from(&input);
    let deer = maze.deer_start.clone();
    let all_solutions = get_all_best_paths(&maze, &deer);
    let mut visits: FxHashSet<Loc> = FxHashSet::default();
    for s in all_solutions.iter() {
        for a in s.actions.iter() {
            match a {
                Action::Move(src, dest) => {
                    visits.insert(src.loc.clone());
                    visits.insert(dest.loc.clone());
                }
                _ => (),
            }
        }
    }
    visits.len()
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn example_day_16_1_1() {
        let path = common::get_test_data_path("day16/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 7036, "found optimal path");
    }

    #[test]
    fn example_day_16_1_2() {
        let path = common::get_test_data_path("day16/case2.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 11048, "found optimal path");
    }

    #[test]
    fn example_day_16_2_1() {
        let path = common::get_test_data_path("day16/case1.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 45, "found optimal path");
    }

    #[test]
    fn example_day_16_2_2() {
        let path = common::get_test_data_path("day16/case2.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 64, "found optimal path");
    }
}
