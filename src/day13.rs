use std::cmp::{min, max};
use std::{fs, thread};
use std::path::PathBuf;
use regex::Regex;

pub mod constants {
    pub const INPUT_PATH: &str = "day13/input.txt";
    pub const A_BTN_COST: isize = 3;
    pub const B_BTN_COST: isize = 1;
    pub const MAX_PRESSES: isize = 100;
}

#[derive(Debug, PartialEq, Clone)]
pub struct Offset {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Loc {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
pub struct ClawConf {
    a_offset: Offset,
    b_offset: Offset,
    prize_location: Loc,
}

#[derive(Clone)]
pub struct SolutionState<'a> {
    num_a_presses: isize,
    num_b_presses: isize,
    conf: &'a ClawConf,
}

impl SolutionState<'_> {
    const A_COST: isize = constants::A_BTN_COST;
    const B_COST: isize = constants::B_BTN_COST;

    pub fn cost(&self) -> isize {
        Self::A_COST * self.num_a_presses + Self::B_COST * self.num_b_presses
    }

    pub fn loc(&self) -> Loc {
        let x = self.num_a_presses * self.conf.a_offset.x
            + self.num_b_presses * self.conf.b_offset.x;

        let y = self.num_a_presses * self.conf.a_offset.y
            + self.num_b_presses * self.conf.b_offset.y;

        Loc { x, y }
    }

    pub fn press_b(&mut self) {
        self.num_b_presses += 1;
    }

    pub fn unpress_b(&mut self) {
        self.num_b_presses -= 1;
    }

    pub fn press_a(&mut self) {
        self.num_a_presses += 1;
    }

    pub fn unpress_a(&mut self) {
        self.num_a_presses -= 1;
    }

    pub fn a_divisible(&self) -> bool {
        let x = self.conf.prize_location.x - self.num_b_presses*self.conf.b_offset.x;
        let y = self.conf.prize_location.y - self.num_b_presses*self.conf.b_offset.y;
        x % self.conf.a_offset.x == 0 && y % self.conf.a_offset.y == 0
    }

    pub fn get_a_presses_needed(&self) -> isize {
        let x = self.conf.prize_location.x - self.num_b_presses*self.conf.b_offset.x;
        x / self.conf.a_offset.x
    }

    pub fn at_target(&self) -> bool {
        let loc = self.loc();
        if loc == self.conf.prize_location {
            true
        } else {
            false
        }
    }

    pub fn beyond_target(&self) -> bool {
        let loc = self.loc();
        loc.x > self.conf.prize_location.x || loc.y > self.conf.prize_location.y
    }
}

pub fn parse_offset(line: &str) -> Offset {
    let rx = Regex::new(r"X\+(\d+), Y\+(\d+)").expect("invalid regex");

    let (_, [x_offset, y_offset]) = rx.captures_iter(&line).map(|c| c.extract()).next().unwrap();
    let x = x_offset.parse::<isize>().unwrap();
    let y = y_offset.parse::<isize>().unwrap();

    Offset { x, y }
}

pub fn parse_loc(line: &str) -> Loc {
    let rx = Regex::new(r"X=(\d+), Y=(\d+)").expect("invalid regex");

    let (_, [x_loc, y_loc]) = rx.captures_iter(&line).map(|c| c.extract()).next().unwrap();
    let x = x_loc.parse::<isize>().unwrap();
    let y = y_loc.parse::<isize>().unwrap();

    Loc { x, y }
}

pub fn parse_conf(input: &String) -> Vec<ClawConf> {
    let mut lines = input.lines();
    let mut result = vec![];

    loop {

        let btn_a_line = lines.next().unwrap();
        let btn_b_line = lines.next().unwrap();
        let prize_loc_line = lines.next().unwrap();

        result.push(ClawConf {
            a_offset: parse_offset(btn_a_line),
            b_offset: parse_offset(btn_b_line),
            prize_location: parse_loc(prize_loc_line),
        });

        if let None = lines.next() {
            break;
        }
    }

    result
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn stop_search(current: &mut Box<SolutionState>, best: &mut Box<SolutionState>, max_press_count: isize) -> bool {
    if current.num_a_presses > max_press_count || current.num_b_presses > max_press_count {
        true
    } else if current.beyond_target() {
        true
    } else if current.cost() >= best.cost() {
        true
    } else {
        false
    }
}

pub fn update_if_at_target(current: &mut Box<SolutionState>, best: &mut Box<SolutionState>) {
    //println!("Target check");
    if current.at_target() {
        //println!("\tPASSED");
        if current.cost() < best.cost() {
            println!("\tCOST UPDATE");
            best.num_a_presses = current.num_a_presses;
            best.num_b_presses = current.num_b_presses;
        } else {
            //println!("\tNO COST UPDATE")
        }
    } else {
        //println!("\tFAILED")
    }
}

pub fn ext_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = ext_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a/b)*y1;
        (gcd, x, y)
    }
}

pub fn solve(a_offset: isize, b_offset: isize, prize_loc: isize) -> Option<(isize, isize, isize)> {
    let (gcd, ao_0, bo_0) = ext_gcd(a_offset, b_offset);

    if prize_loc % gcd != 0 {
        return None;
    }

    let factor = prize_loc / gcd;
    let x = ao_0*factor;
    let y = bo_0*factor;

    Some((gcd, x, y))
}

//pub fn _solution_exists(a_offset: isize, b_offset: isize, prize_loc: isize) -> bool {
//    let offset_gcd = gcd(a_offset, b_offset);
//    prize_loc % offset_gcd == 0
//}
//
//pub fn solution_exists(current: &Box<SolutionState>) ->  bool {
//    let a_offset = &current.conf.a_offset;
//    let b_offset = &current.conf.b_offset;
//    let prize_loc = &current.conf.prize_location;
//    _solution_exists(a_offset.x, b_offset.x, prize_loc.x)
//        && _solution_exists(a_offset.y, b_offset.y, prize_loc.y)
//}

pub fn _find_min_winning_cost(current: &mut Box<SolutionState>, best: &mut Box<SolutionState>, max_press_count: isize) {
    for b_press in 0..max_press_count {
        if stop_search(current, best, max_press_count) {
            break;
        } else {
            update_if_at_target(current, best);
            if current.a_divisible() {
                current.num_a_presses = current.get_a_presses_needed();
                update_if_at_target(current, best);
            }
        }
        current.num_a_presses = 0;
        current.press_b();
    }
}

pub fn find_valid_k_range(x_0: isize, x_scale: isize, y_0: isize, y_scale: isize) ->  std::ops::RangeInclusive<isize> {
    let min_x_k = ((-x_0 as f64) / (x_scale as f64)).ceil() as isize;
    let min_y_k = ((y_0 as f64) / (y_scale as f64)).floor() as isize;
    let bigger_k = max(min_x_k, min_y_k);
    let smaller_k = min(min_x_k, min_y_k);
    smaller_k..=bigger_k
}

pub fn find_min_winning_cost(conf: &ClawConf, max_presses: isize) -> Option<isize> {
    println!("{:#?}", conf);
    let mut best = Box::new(SolutionState {
        conf,
        num_b_presses: max_presses + 1,
        num_a_presses: max_presses + 1,
    });
    
    let mut init = Box::new(SolutionState {
        conf,
        num_b_presses: 0,
        num_a_presses: 0
    });

    match solve(conf.a_offset.x, conf.b_offset.x, conf.prize_location.x) {
        Some((gcd_x, a_moves_x, b_moves_x)) => {
            println!("found X solution");
            //println!(
            //    "GCD({}, {}) = {}",
            //    conf.a_offset.x,
            //    conf.b_offset.x,
            //    gcd_x,
            //);
            //println!(
            //    "a_x = {} + k*({})",
            //    a_moves_x,
            //    conf.b_offset.x / gcd_x,
            //);
            //println!(
            //    "b_x = {} - k*({})",
            //    b_moves_x,
            //    conf.a_offset.x / gcd_x,
            //);
            let valid_k_x = find_valid_k_range(
                a_moves_x,
                conf.b_offset.x/gcd_x,
                b_moves_x,
                conf.a_offset.x/gcd_x,
            );
            match solve(conf.a_offset.y, conf.b_offset.y, conf.prize_location.y) {
                Some((gcd_y, a_moves_y, b_moves_y)) => {
                    println!("Y solution found");
                    let valid_k_y = find_valid_k_range(
                        a_moves_y,
                        conf.b_offset.y/gcd_y,
                        b_moves_y,
                        conf.a_offset.y/gcd_y,
                    );
                    let x_range_len = valid_k_x.end() - valid_k_x.start();
                    let y_range_len = valid_k_y.end() - valid_k_y.start();
                    if x_range_len < y_range_len {
                        for k in valid_k_x {
                            //println!("");
                            let num_a_presses = a_moves_x + k*(conf.b_offset.x/gcd_x);
                            let num_b_presses = b_moves_x - k*(conf.a_offset.x/gcd_x);


                            init.num_a_presses = a_moves_x + k*(conf.b_offset.x/gcd_x);
                            init.num_b_presses = b_moves_x - k*(conf.a_offset.x/gcd_x);
                            //println!("\t(X) a presses: {}", init.num_a_presses);
                            //println!("\t(X) b presses: {}", init.num_b_presses);
                            update_if_at_target(&mut init, &mut best);
                        }
                    } else {
                        for k in valid_k_y {
                            //println!("");
                            init.num_a_presses = a_moves_y + k*(conf.b_offset.y/gcd_y);
                            init.num_b_presses = b_moves_y - k*(conf.a_offset.y/gcd_y);
                            //println!("\t(Y) a presses: {}", init.num_a_presses);
                            //println!("\t(Y) b presses: {}", init.num_b_presses);
                            update_if_at_target(&mut init, &mut best);
                        }

                    }
                },
                None => {
                    println!("no Y solution found");
                },
            }
        },
        None => {
            println!("no X solution found");
        },
    }

    //if solution_exists(&mut init) {
    //    //_find_min_winning_cost(&mut init, &mut best, max_presses);
    //}

    if best.at_target() {
        Some(best.cost())
    } else {
        None
    }
}

pub fn _solution1(input: &String) -> isize {
    let confs = parse_conf(input);
    let mut token_count = 0;
    for c in confs.iter() {
        match find_min_winning_cost(c, constants::MAX_PRESSES) {
            Some(cost) => token_count += cost,
            None => (),
        }
    }
    token_count
}

pub fn solution2(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> isize {
    let offset_adjustment = 10000000000000;
    let max_steps = offset_adjustment;
    let mut confs = parse_conf(input);
    for c in confs.iter_mut() {
        c.prize_location.x += offset_adjustment;
        c.prize_location.y += offset_adjustment;
    }
    let chunk_size = (confs.len() + 15) / 16;
    let mut handles = vec![];
    for conf_chunk in confs.chunks(chunk_size) {
        let chunk = conf_chunk.to_vec();
        let mut token_count = 0;
        let handle = thread::spawn(move || {
            for c in chunk {
                match find_min_winning_cost(&c, max_steps) {
                    Some(cost) => token_count += cost,
                    None => (),
                }
            }
            token_count
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn example_day_13_1_parse_input() {
        let path = common::get_test_data_path("day13/case1.txt").unwrap();
        let input =  fs::read_to_string(path).unwrap();
        let configs = parse_conf(&input);

        assert_eq!(configs.len(), 4, "parsed all configs");
        let c = &configs[0];
        assert_eq!(c.a_offset, Offset { x: 94, y: 34 }, "correct a offset");
        assert_eq!(c.b_offset, Offset { x: 22, y: 67 }, "correct b offset");
        assert_eq!(c.prize_location, Loc { x: 8400, y: 5400 }, "correct price loc");
    }

    #[test]
    fn example_day_13_1() {
        let path = common::get_test_data_path("day13/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 480);
    }

    #[test]
    fn example_day_13_2() {
        let path = common::get_test_data_path("day13/case1.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 480);
    }
}
