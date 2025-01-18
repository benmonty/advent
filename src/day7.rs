use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;
use num_bigint::BigUint;

#[derive(Clone)]
pub enum Operator {
    Plus,
    Mult,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EqData {
    solution: u128,
    operands: Vec<u128>,
}

pub mod constants {
    pub const INPUT_PATH: &str = "day7/input.txt";
}

pub fn solution1(path: &PathBuf) -> BigUint {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

fn parse_input(input: &String) -> Vec<EqData> {
    let mut eq_data = Vec::new();
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(": ").collect();
        assert_eq!(line_parts.len(), 2, "expecting result and operands");
        let solution = line_parts[0].parse::<u128>().unwrap();
        let operands: Vec<u128> = line_parts[1].split(" ").map(|s| s.parse::<u128>().unwrap()).collect();
        eq_data.push(EqData{
            solution,
            operands,
        });
    }
    eq_data
}

fn _find_op_sequence(eq_data: &EqData, valid_ops: &Vec<Operator>, acc: u128, remaining: &mut VecDeque<u128>, ops: &mut Vec<Operator>) -> Option<Vec<Operator>> {
    if remaining.len() == 0 {
        if acc == eq_data.solution {
            return Some(ops.to_vec());
        } else {
            return None;
        }
    } else if acc > eq_data.solution {
        return None;
    }
    for op in valid_ops.iter() {
        let next_acc;
        let is_overflow;
        let next_operand = remaining.pop_front().unwrap();
        ops.push(op.clone());
        match op {
            Operator::Plus => {
                (next_acc, is_overflow)  = acc.overflowing_add(next_operand);

                // assume that any operators that overflow usize
                // will result in no solution
                if is_overflow {
                    println!("overflowing add");
                }
            },
            Operator::Mult => {
                (next_acc, is_overflow) = acc.overflowing_mul(next_operand);

                if is_overflow {
                    println!("overflowing mult");
                }
            },
        }
        match _find_op_sequence(eq_data, valid_ops, next_acc, remaining, ops) {
            Some(result) => { return Some(result); },
            None => (),
        }
        remaining.push_front(next_operand);
        ops.pop();
    }
    None
}

pub fn find_op_sequence(eq_data: &EqData, valid_ops: &Vec<Operator>) -> Option<Vec<Operator>> {
    let init_acc = eq_data.operands[0];
    let mut rest: VecDeque<u128> = eq_data.operands[1..].iter().cloned().collect();
    let mut ops: Vec<Operator> = Vec::new();
    _find_op_sequence(eq_data, valid_ops, init_acc, &mut rest, &mut ops)
}

//pub fn check_solution(eq_data: &EqData, ops: &Vec<Operator>) {
//    let mut acc = eq_data.operands[0];
//    for (i, operand) in eq_data.operands[1..].iter().enumerate() {
//        match ops[i] {
//            Operator::Mult => acc *= operand,
//            Operator::Plus => acc += operand,
//        }
//    }
//    assert_eq!(acc, eq_data.solution)
//}
//use rand::Rng;
//pub fn rand_find_match(eq_data: &EqData, valid_ops: &Vec<Operator>) -> bool {
//    let operands = eq_data.operands.clone();
//    let mut rng = rand::thread_rng();
//    for i in 0..10000 {
//        let mut acc = operands[0];
//        for j in &operands[1..] {
//            match rng.gen_range(0..=1) {
//                0 => acc += j,
//                1 => acc *= j,
//                _ => assert!(false),
//            }
//        }
//        if eq_data.solution == acc {
//            return true;
//        }
//    }
//    false
//}

pub fn _solution1(input: &String) -> BigUint {
    let test_eqs = parse_input(&input);
    println!("found {} equations", test_eqs.len());
    let ops = vec![Operator::Plus, Operator::Mult];
    let mut result = BigUint::from(0u64);
    let mut r = 0u128;
    let mut is_overflow;
    for test_eq in test_eqs.iter() {
        //if rand_find_match(&test_eq, &ops) {
        //    let result_part = BigUint::from(test_eq.solution);
        //    result += result_part;
        //    match find_op_sequence(&test_eq, &ops) {
        //        None => {
        //            println!("BAD CASE: {}", test_eq.solution);
        //        }
        //        _ => (),
        //    }
        //}
        match find_op_sequence(&test_eq, &ops) {
            Some(_op_seq) => {
                //check_solution(&test_eq, &_op_seq);
                let result_part = BigUint::from(test_eq.solution);
                result += result_part;
                (r, is_overflow) = r.overflowing_add(test_eq.solution);
                println!("{}\t{}", result, is_overflow);
                assert!(!is_overflow, "is overflowx");
            },
            _ => (),
        }
    }
    result
}

pub fn solution2(path: &PathBuf) -> u128 {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> u128 {
    0
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_parse_input() {
        let input = String::from("5: 1 2 3\n4: 5 6 7");
        let parsed = parse_input(&input);
        assert_eq!(
            parsed,
            vec![
                EqData {
                    solution: 5,
                    operands: vec![1, 2, 3],
                },
                EqData {
                    solution: 4,
                    operands: vec![5, 6, 7],
                },
            ]
        )
    }

    #[test]
    fn test_solution1() {
        let path = common::get_test_data_path("day7/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, 3749);
    }
    
    //#[test]
    //fn test_solution2() {
    //    assert!(false, "todo")
    //}
}
