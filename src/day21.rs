use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Key {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
pub enum ArmOp {
    Up,
    Down,
    Left,
    Right,
    Press,
}

pub struct Keypad {
    links: FxHashMap<Key, FxHashMap<ArmOp, Key>>,
}

fn get_link_map(raw_links: Vec<(ArmOp, Key)>) -> FxHashMap<ArmOp, Key> {
    let mut m = FxHashMap::default();
    for raw_link in raw_links.iter() {
        m.entry(raw_link.0.clone()).or_insert(raw_link.1.clone());
    }
    m
}

impl Keypad {

    pub fn is_numeric(&self) -> bool {
        self.links.contains_key(&Key::Zero)
    }

    pub fn create_numeric() -> Self {
        let mut links = FxHashMap::default();

        links.entry(Key::A).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Three),
            (ArmOp::Left, Key::Zero),
        ]));

        links.entry(Key::Zero).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Two),
            (ArmOp::Right, Key::A),
        ]));

        links.entry(Key::One).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Four),
            (ArmOp::Right, Key::Two),
        ]));

        links.entry(Key::Two).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Five),
            (ArmOp::Left, Key::One),
            (ArmOp::Right, Key::Three),
            (ArmOp::Down, Key::Zero),
        ]));

        links.entry(Key::Three).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Six),
            (ArmOp::Left, Key::Two),
            (ArmOp::Down, Key::A),
        ]));

        links.entry(Key::Four).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Seven),
            (ArmOp::Down, Key::One),
            (ArmOp::Right, Key::Five),
        ]));

        links.entry(Key::Five).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Eight),
            (ArmOp::Down, Key::Two),
            (ArmOp::Left, Key::Four),
            (ArmOp::Right, Key::Six),
        ]));

        links.entry(Key::Six).or_insert(get_link_map(vec![
            (ArmOp::Up, Key::Nine),
            (ArmOp::Down, Key::Three),
            (ArmOp::Left, Key::Five),
        ]));

        links.entry(Key::Seven).or_insert(get_link_map(vec![
            (ArmOp::Down, Key::Four),
            (ArmOp::Right, Key::Eight),
        ]));

        links.entry(Key::Eight).or_insert(get_link_map(vec![
            (ArmOp::Down, Key::Five),
            (ArmOp::Left, Key::Seven),
            (ArmOp::Right, Key::Nine),
        ]));

        links.entry(Key::Nine).or_insert(get_link_map(vec![
            (ArmOp::Down, Key::Six),
            (ArmOp::Left, Key::Eight),
        ]));

        Self {
            links,
        }
    }

    pub fn create_directional() -> Self {
        let mut links = FxHashMap::default();

        links.entry(Key::A).or_insert(get_link_map(vec![
            (ArmOp::Left, Key::Up),
            (ArmOp::Down, Key::Right),
        ]));

        links.entry(Key::Up).or_insert(get_link_map(vec![
            (ArmOp::Down, Key::Down),
            (ArmOp::Right, Key::A),
        ]));

        links.entry(Key::Right).or_insert(get_link_map(vec![
            (ArmOp::Left, Key::Down),
            (ArmOp::Up, Key::A),
        ]));

        links.entry(Key::Down).or_insert(get_link_map(vec![
            (ArmOp::Left, Key::Left),
            (ArmOp::Right, Key::Right),
            (ArmOp::Up, Key::Up),
        ]));

        links.entry(Key::Left).or_insert(get_link_map(vec![
            (ArmOp::Right, Key::Down),
        ]));

        Self {
            links,
        }
    }
}

pub struct RobotController {
    keypad: Keypad,
    child: Option<Box<RobotController>>,
}

pub fn op_to_key(op: &ArmOp) -> Key {
    match op {
        ArmOp::Up => Key::Up,
        ArmOp::Down => Key::Down,
        ArmOp::Left => Key::Left,
        ArmOp::Right => Key::Right,
        ArmOp::Press => Key::A,
    }
}

impl RobotController {
    pub fn solve_root_seq(&self, key_seq: &Vec<Key>) -> Vec<Vec<Vec<ArmOp>>> {
        match &self.child {
            Some(controller) => {
                let key_paths_to_solve = controller.solve_root_seq(&key_seq);
                let mut result = vec![];
                for key_group_paths in key_paths_to_solve.iter() {
                    //println!("KEY_GROUP_PATHS");
                    //println!("{:#?}", key_group_paths);
                    // [Up, Press]
                    let mut key_group_results: Vec<Vec<ArmOp>> = vec![];

                    for key_path in key_group_paths.iter() {
                        //println!("KEY_PATH");
                        //println!("{:#?}", key_path);
                        let mut pos = Key::A;
                        let mut key_path_results: Vec<Vec<ArmOp>> = vec![];
                        for op in key_path.iter() {
                            let k = op_to_key(&op);
                            let mut shortest_paths = shortest_paths(&self.keypad, &pos, &k);
                            for i in 0..shortest_paths.len() {
                                shortest_paths[i].push(ArmOp::Press);
                            }
                            if key_path_results.len() > 0 {
                                let mut next_key_path_results = vec![];
                                for i in 0..key_path_results.len() {
                                    for j in 0..shortest_paths.len() {
                                        let mut kp = key_path_results[i].clone();
                                        let mut sp = shortest_paths[j].clone();
                                        kp.append(&mut sp);

                                        next_key_path_results.append(&mut vec![kp]);
                                    }
                                }
                                key_path_results = next_key_path_results;
                            } else {
                                key_path_results.append(&mut shortest_paths);
                            }
                            pos = k.clone();
                        }
                        key_group_results.append(&mut key_path_results);
                    }
                    result.append(&mut vec![key_group_results]);
                }
                //println!("NON-ROOT");
                //println!("{:#?}", result);
                return result;
            },
            None => {
                let mut result = vec![];
                let mut pos = Key::A;
                for k in key_seq.iter() {
                    let mut shortest_paths = shortest_paths(&self.keypad, &pos, &k);
                    for i in 0..shortest_paths.len() {
                        shortest_paths[i].push(ArmOp::Press);
                    }
                    result.append(&mut vec![shortest_paths]);
                    pos = k.clone();
                }
                //println!("ROOT");
                //println!("{:#?}", result);
                return result;
            }
        };
    }
}

pub fn _best_paths(keypad: &Keypad, src_key: &Key, dest_key: &Key, best_paths: &mut FxHashMap<Key, Vec<Vec<ArmOp>>>) {
    if *src_key == *dest_key {
        return;
    }
    let mut next_src_keys: Vec<Key> = vec![];
    let src_bests = best_paths.get(&src_key).unwrap().clone();
    for (dir, key) in keypad.links.get(src_key).unwrap().iter() {
        //dbg!(dir, key);
        if best_paths.contains_key(&key) {
            let existing_bests = best_paths.get(&key).unwrap().clone();
            if src_bests[0].len() + 1 < existing_bests[0].len() {
                let mut current_bests = src_bests.clone();
                //println!("replace new best");
                //dbg!(&current_bests);
                for i in 0..current_bests.len() {
                    current_bests[i].push(dir.clone());
                }
                next_src_keys.push(key.clone());
                best_paths.entry(key.clone()).and_modify(|b| *b = current_bests);
                //dbg!(best_paths.get(&key.clone()).unwrap());
            } else if src_bests[0].len() + 1 == existing_bests[0].len() {
                //println!("add new best");
                let mut current_bests = src_bests.clone();
                let mut to_add_bests = vec![];
                //dbg!(&current_bests);
                for i in 0..current_bests.len() {
                    current_bests[i].push(dir.clone());
                    if !best_paths.get(&key.clone()).unwrap().contains(&current_bests[i]) {
                        to_add_bests.push(current_bests[i].clone());
                    }
                }
                best_paths.entry(key.clone()).and_modify(|b| b.append(&mut to_add_bests));
                //dbg!(best_paths.get(&key.clone()).unwrap());
                next_src_keys.push(key.clone());
            }
        } else {
            let mut current_bests = src_bests.clone();
            for i in 0..current_bests.len() {
                current_bests[i].push(dir.clone());
            }
            best_paths.entry(key.clone()).or_insert(current_bests);
            next_src_keys.push(key.clone());
        }
        //println!("");
    }
    for key in next_src_keys.iter() {
        _best_paths(&keypad, &key, &dest_key, best_paths);
    }
}

pub fn shortest_paths(keypad: &Keypad, src_key: &Key, dest_key: &Key) -> Vec<Vec<ArmOp>> {
    let mut best_paths: FxHashMap<Key, Vec<Vec<ArmOp>>> = FxHashMap::default();
    best_paths.entry(src_key.clone()).or_insert(vec![vec![]]);

    _best_paths(&keypad, &src_key, &dest_key, &mut best_paths);

    let mut min_directional_changes = isize::MAX;
    if best_paths.get(&dest_key).unwrap()[0].len() == 0 {
        return best_paths.get(&dest_key).unwrap().clone();
    }
    for p in best_paths.get(&dest_key).unwrap().iter() {
        let mut dir_changes = 0;
        let mut last_d = &p[0];
        for d in p[1..].iter() {
            if last_d != d {
                dir_changes += 1;
            }
            last_d = d;
        }
        if dir_changes < min_directional_changes {
            min_directional_changes = dir_changes;
        }
    }

    let mut result = vec![];
    for p in best_paths.get(&dest_key).unwrap().iter() {
        let mut dir_changes = 0;
        let mut last_d = &p[0];
        for d in p[1..].iter() {
            if last_d != d {
                dir_changes += 1;
            }
            last_d = d;
        }
        if dir_changes == min_directional_changes {
            result.push(p.clone());
        }
    }

    result
}


pub mod constants {
    pub const INPUT_PATH: &str = "day21/input.txt";
}

pub fn parse_keys(code: &String) -> Vec<Key> {
    let mut result = vec![];
    for c in code.chars() {
        let k = match c {
            '0' => Key::Zero,
            '1' => Key::One,
            '2' => Key::Two,
            '3' => Key::Three,
            '4' => Key::Four,
            '5' => Key::Five,
            '6' => Key::Six,
            '7' => Key::Seven,
            '8' => Key::Eight,
            '9' => Key::Nine,
            'A' => Key::A,
            _ => panic!("unknown key"),
        };
        result.push(k);
    }
    result
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    let mut codes = vec![];

    for line in input.lines() {
        codes.push(parse_keys(&line.to_string()));
    }

    _solution1(&codes)
}

pub fn numeric_complexity_component(code: &Vec<Key>) -> isize {
    let mut mult = 100;
    let mut result = 0;
    for i in 0..3 {
        let factor = match code[i] {
            Key::Zero => 0,
            Key::One => 1,
            Key::Two => 2,
            Key::Three => 3,
            Key::Four => 4,
            Key::Five => 5,
            Key::Six => 6,
            Key::Seven => 7,
            Key::Eight => 8,
            Key::Nine => 9,
            _ => panic!("unexpected non-numeric key"),
        };
        result += mult * factor;
        mult /= 10;
    }
    result
}

pub fn render_one(ops: &Vec<ArmOp>) -> String {
    let mut result = String::new();
    for op in ops.iter() {
        result.push(match op {
            ArmOp::Up => '^',
            ArmOp::Down => 'v',
            ArmOp::Left => '<',
            ArmOp::Right => '>',
            ArmOp::Press => 'A',
        });
    }
    result
}

pub fn render_all(ops: &Vec<Vec<Vec<ArmOp>>>) {
    for first in ops[0].iter() {
        for second in ops[1].iter() {
            for third in ops[2].iter() {
                for fourth in ops[3].iter() {
                    println!(
                        "{}|{}|{}|{}",
                        render_one(&first),
                        render_one(&second),
                        render_one(&third),
                        render_one(&fourth),
                    )
                }
            }
        }
    }
}

pub fn compute_complexity(controller: &RobotController, codes: &Vec<Vec<Key>>) -> isize {
    let mut total_complexity = 0;
    for code in codes.iter() {
        dbg!(&code);
        let mut total_best = 0;
        let key_solutions = controller.solve_root_seq(&code);
        //render_all(&key_solutions);
        for i in 0..key_solutions.len() {
            let mut best = key_solutions[i][0].len();
            for j in 0..key_solutions[i].len() {
                if key_solutions[i][j].len() < best {
                    best = key_solutions[i][j].len();
                }
            }
            total_best += best;
        }
        let shortest_seq_len = isize::try_from(total_best).unwrap();
        println!("shortest: {}", shortest_seq_len);
        let numeric = numeric_complexity_component(&code);
        println!("numeric: {}", numeric);
        total_complexity += shortest_seq_len * numeric;
        println!("");
    }
    total_complexity
}

pub fn _solution1(codes: &Vec<Vec<Key>>) -> isize {
    let numeric_controller = RobotController {
        keypad: Keypad::create_numeric(),
        child: None,
    };
    let dir_controller1 = RobotController {
        keypad: Keypad::create_directional(),
        child: Some(Box::new(numeric_controller)),
    };
    let dir_controller2 = RobotController {
        keypad: Keypad::create_directional(),
        child: Some(Box::new(dir_controller1)),
    };
    compute_complexity(&dir_controller2, &codes)
}

pub fn solution2(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    let mut codes = vec![];

    for line in input.lines() {
        codes.push(parse_keys(&line.to_string()));
    }
    _solution2(&codes)
}

pub fn _solution2(codes: &Vec<Vec<Key>>) -> isize {
    let mut controller = RobotController {
        keypad: Keypad::create_numeric(),
        child: None,
    };
    for _i in 0..25 {
        controller = RobotController {
            keypad: Keypad::create_directional(),
            child: Some(Box::new(controller))
        }
    }
    compute_complexity(&controller, &codes)
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn day_21_1_1_numeric() {
        let kp = Keypad::create_numeric();
        {
            let paths = shortest_paths(&kp, &Key::A, &Key::Nine);
            assert_eq!(
                paths,
                vec![
                    vec![ArmOp::Up, ArmOp::Up, ArmOp::Up],
                ]
            );
        }
        {
            let paths = shortest_paths(&kp, &Key::A, &Key::One);
            assert_eq!(
                paths,
                vec![
                    vec![ArmOp::Up, ArmOp::Left, ArmOp::Left],
                    vec![ArmOp::Left, ArmOp::Up, ArmOp::Left],
                ]
            );
        }
        {
            let paths = shortest_paths(&kp, &Key::One, &Key::A);
            assert_eq!(
                paths,
                vec![
                    vec![ArmOp::Right, ArmOp::Down, ArmOp::Right],
                    vec![ArmOp::Right, ArmOp::Right, ArmOp::Down],
                ]
            );
        }
        {
            let paths = shortest_paths(&kp, &Key::Nine, &Key::Four);
            assert_eq!(
                paths,
                vec![
                    vec![ArmOp::Left, ArmOp::Left, ArmOp::Down],
                    vec![ArmOp::Left, ArmOp::Down, ArmOp::Left],
                    vec![ArmOp::Down, ArmOp::Left, ArmOp::Left],
                ]
            );
        }
    }

    //#[test]
    //fn day_21_1_1_directional() {
    //    let kp = Keypad::create_directional();
    //    {
    //        let path = shortest_path(&kp, &Key::A, &Key::Left);
    //        assert_eq!(path, vec![ArmOp::Down, ArmOp::Left, ArmOp::Left]);
    //    }
    //    {
    //        let path = shortest_path(&kp, &Key::Left, &Key::A);
    //        assert_eq!(path, vec![ArmOp::Right, ArmOp::Right, ArmOp::Up]);
    //    }
    //}

    #[test]
    fn day_21_1_2() {
        let numeric_controller = RobotController {
            keypad: Keypad::create_numeric(),
            child: None,
        };
        let dir_controller1 = RobotController {
            keypad: Keypad::create_directional(),
            child: Some(Box::new(numeric_controller)),
        };
        let dir_controller2 = RobotController {
            keypad: Keypad::create_directional(),
            child: Some(Box::new(dir_controller1)),
        };
        let code = vec![Key::Zero, Key::Two, Key::Nine, Key::A];
        let result = dir_controller2.solve_root_seq(&code);
        assert_eq!(result.len(), 68);
    }

    #[test]
    fn day_21_1_3() {
        let result = _solution1(&vec![
            vec![Key::Zero, Key::Two, Key::Nine, Key::A],
            vec![Key::Nine, Key::Eight, Key::Zero, Key::A],
            vec![Key::One, Key::Seven, Key::Nine, Key::A],
            vec![Key::Four, Key::Five, Key::Six, Key::A],
            vec![Key::Three, Key::Seven, Key::Nine, Key::A],
        ]);
        assert_eq!(result, 126384);
    }

    #[test]
    fn day_21_2_1() {
        assert!(false, "todo")
    }
}
