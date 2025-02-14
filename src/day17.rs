use std::fs;
use std::path::PathBuf;
use std::thread;
use std::collections::VecDeque;

use rustc_hash::{FxHashMap,FxHashSet};

pub mod constants {
    pub const INPUT_PATH: &str = "day17/input.txt";
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<isize>,
}

impl Program {
    pub fn from(input: &String) -> Self {
        let mut instructions: Vec<isize> = vec![];

        for line in input.lines() {
            if line.starts_with("Program: ") {
                let (_, raw_ins) = line.split_once(" ").unwrap();
                for i in raw_ins.split(',') {
                    instructions.push(i.parse::<isize>().unwrap());
                }
            }
        }

        Self {
            instructions,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Proc {
    reg_a: isize,
    reg_b: isize,
    reg_c: isize,
    debug: bool,
    ins_ptr: isize,
    output: Vec<isize>,
}

pub struct Combo(isize);
pub struct Literal(isize);
pub struct Ignored(isize);

pub enum Op {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc(Ignored),
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

pub fn to_u(i: isize) -> usize {
    usize::try_from(i).unwrap()
}

pub fn to_i(u: usize) -> isize {
    isize::try_from(u).unwrap()
}

impl Proc {

    pub fn _debug(&self, s: String) {
        if self.debug {
            println!("{}", s);
        }
    }

    pub fn _valid_op(&self, ins_ptr: isize, program: &Program) -> bool {
        if ins_ptr < 0 || ins_ptr >= isize::try_from(program.instructions.len()).unwrap() {
            return false;
        }
        let ins = program.instructions[to_u(ins_ptr)];
        match  ins {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let operand_ptr = ins_ptr + 1;
                if operand_ptr >= to_i(program.instructions.len()) {
                    return false;
                }
                true
            },
            _ => false,
        }

    }

    pub fn resolve_combo(&self, operand: &Combo) -> isize {
        match operand.0 {
            0 | 1 | 2 | 3 => operand.0,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("will not appear in valid program"),
            i => panic!("unknown combo operand: {}", i),
        }
    }

    pub fn resolve_literal(&self, operand: &Literal) -> isize {
        operand.0
    }

    pub fn op(&self, ins_ptr: isize, program: &Program) -> Option<Op> {
        if !self._valid_op(ins_ptr, &program) {
            return None;
        }
        let op_code = program.instructions[to_u(ins_ptr)];
        let operand_idx = to_u(ins_ptr + 1);
        match op_code {
            0 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Adv(Combo(operand)))
            },
            1 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Bxl(Literal(operand)))
            }
            2 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Bst(Combo(operand)))
            }
            3 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Jnz(Literal(operand)))
            }
            4 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Bxc(Ignored(operand)))
            }
            5 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Out(Combo(operand)))
            }
            6 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Bdv(Combo(operand)))
            }
            7 => {
                let operand = program.instructions[operand_idx];
                Some(Op::Cdv(Combo(operand)))
            }
            _ => None,
        }
    }

    pub fn new(reg_a: isize, reg_b: isize, reg_c: isize) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            debug: false,
            ins_ptr: 0,
            output: vec![],
        }
    }

    pub fn from(input: &String) -> Self {
        let mut reg_a: isize = isize::MAX;
        let mut reg_b: isize = isize::MAX;
        let mut reg_c: isize = isize::MAX;

        for line in input.lines() {
            if line.contains("Register") {
                let (reg_label, raw_val) = line.split_once(": ").unwrap();
                let val = raw_val.parse::<isize>().unwrap();

                match reg_label {
                    "Register A" => {
                        reg_a = val;
                    },
                    "Register B" => {
                        reg_b = val;
                    },
                    "Register C" => {
                        reg_c = val;
                    },
                    _ => (),
                }

            }
        }
        Self {
            reg_a,
            reg_b,
            reg_c,
            debug: false,
            ins_ptr: 0,
            output: vec![],
        }

    }
    
    pub fn step(&mut self, program: &Program) -> Option<isize> {
        if let Some(op) = self.op(self.ins_ptr, &program) {
            match op {
                Op::Adv(combo) => {
                    let operand = self.resolve_combo(&combo);
                    let numerator = self.reg_a;
                    let denominator = 1 << operand;
                    //self._debug(format!("ADV: combo({})\tnumerator({})\tdenominator({})", operand, numerator, denominator));
                    self.reg_a = numerator/denominator;
                    Some(self.ins_ptr + 2)
                },
                Op::Bxl(lit) => {
                    let operand = self.resolve_literal(&lit);
                    let result = self.reg_b ^ operand;
                    //self._debug(format!("BXL: lit({})\treg_b({})\tresult({})", operand, self.reg_b, result));
                    self.reg_b = result;
                    Some(self.ins_ptr + 2)
                }
                Op::Bst(combo) => {
                    let operand = self.resolve_combo(&combo);
                    let result = operand % 8;
                    //self._debug(format!("BST: combo({})\tresult({})", operand, result));
                    self.reg_b = result;
                    Some(self.ins_ptr + 2)
                }
                Op::Jnz(lit) => {
                    //self._debug(format!("JNZ: reg_a({})", self.reg_a));
                    if self.reg_a == 0 {
                        //self._debug(format!("\tno jump"));
                        Some(self.ins_ptr + 2)
                    } else {
                        let next_ptr = self.resolve_literal(&lit);
                        //self._debug(format!("\tnext_ptr({})", next_ptr));
                        Some(next_ptr)
                    }
                }
                Op::Bxc(_) => {
                    let result = self.reg_b ^ self.reg_c;
                    //self._debug(format!("BXC: regb({})\tregc({})\tresult({})", self.reg_b, self.reg_c, result));
                    self.reg_b = result;
                    Some(self.ins_ptr + 2)
                }
                Op::Out(combo) => {
                    let operand = self.resolve_combo(&combo);
                    let result = operand % 8;
                    self.output.push(result);
                    //self._debug(format!("OUT: combo({})\tresult({})", operand, result));
                    Some(self.ins_ptr + 2)
                }
                Op::Bdv(combo) => {
                    let operand = self.resolve_combo(&combo);
                    let numerator = self.reg_a;
                    let denominator = 1 << operand;
                    self.reg_b = numerator/denominator;
                    //self._debug(format!("BDV: combo({})\tnumerator({})\tdenominator({})", operand, numerator, denominator));
                    Some(self.ins_ptr + 2)
                }
                Op::Cdv(combo) => {
                    let operand = self.resolve_combo(&combo);
                    let numerator = self.reg_a;
                    let denominator = 1 << operand;
                    self.reg_c = numerator/denominator;
                    self._debug(format!("CDV: combo({})\tnumerator({})\tdenominator({})", operand, numerator, denominator));
                    Some(self.ins_ptr + 2)
                }
            }
        } else {
            None
        }

    }

    pub fn execute(&mut self, program: &Program) -> Vec<isize> {
        loop {
            match self.step(&program) {
                Some(next_ins_ptr) => {
                    self.ins_ptr = next_ins_ptr;
                },
                None => break,
            }
        }
        self.output.clone()
    }

    pub fn execute_step(&mut self, program: &Program ) -> Option<Vec<isize>> {
        match self.step(&program) {
            Some(next_ins_ptr) => {
                self.ins_ptr = next_ins_ptr;
                None
            },
            None => Some(self.output.clone()),
        }
    }
}

pub fn solution1(path: &PathBuf) -> String {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub fn _solution1(input: &String) -> String {
    let mut proc = Proc::from(&input);
    let prog = Program::from(&input);
    println!("{:#?}", proc);
    println!("{:#?}", prog);
    let output = proc.execute(&prog);
    println!("{:#?}", proc);
    let output_as_strs: Vec<String> = output.iter().map(|i| i.to_string()).collect();
    output_as_strs.join(",")
}

pub fn solution2(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}


/*

x0 = 1-7
x1 = 8*x0
x2 = 8*x1
x3 = 8*x2
x4 = 8*x3

B = (A % 8) ^ 5
    0 000 ^ 101 = 101 = 5
    1 001 ^ 101 = 100 = 4
    2 010 ^ 101 = 111 = 7
    3 011 ^ 101 = 110 = 6
    4 100 ^ 101 = 001 = 1
    5 101 ^ 101 = 000 = 0
    6 110 ^ 101 = 011 = 3
    7 111 ^ 101 = 010 = 2

C = A / 1 << ((A % 8)^5) # A / 2**(0-7) # A/{1, 2, 4, 8, 16, 32, 64, 128}

B = ((A % 8) ^ 5) ^ 6
    0 000 ^ 110 = 110 = 6
    1 001 ^ 110 = 111 = 7
    2 010 ^ 110 = 100 = 4
    3 011 ^ 110 = 101 = 5
    4 100 ^ 110 = 010 = 2
    5 101 ^ 110 = 011 = 3
    6 110 ^ 110 = 000 = 0
    7 111 ^ 110 = 001 = 1

B = B ^ C

B = b^(A/2**n)

OUT (B^C) % 8

(B ^ C) % 8 == 2
x0x ^ x1x OR y1y ^ y0y
B_2 AND C MUST
x0x ^ x1x case
    B_2
        101 5
    -> B_1
        011 3
    -> A_8
        5
    -> A % 8 must be 6
    C
        A / (1 << B_1)
        needs to be x1x
        A / (1 << B_1) must be 7
        if B_1 is 3
            (A / (1 << 3)) % 8 is 7
    B_2
        000 0
    -> B_1
        110 6
    -> A % A % 8
        011 3
    C
        A / (1 << B_1) % 8
        needs to be 010 (2)
        A / (1 << 6)
        A / 64 % 8 == 2
            
        


Program:
2,4
B = (A % 8)
1,5
B = ((A % 8) ^ 5)
7,5
C = ((A) / (1 << ((A % 8) ^ 5)))
1,6
B = (((A % 8) ^ 5) ^ 6)
4,2
B = ((((A % 8) ^ 5) ^ 6) ^ ((A) / (1 << ((A % 8) ^ 5))))
5,5
OUT: (((((A % 8) ^ 5) ^ 6) ^ ((A) / (1 << ((A % 8) ^ 5)))) % 8)
0,3
A = ((A) / (1 << 3))
3,0

*/

pub fn _test_a_val(input: &String, a: isize) {
    let mut proc = Proc::from(&input);
    let prog = Program::from(&input);
    proc.reg_a = a;
    let output = proc.execute(&prog);
    println!("{:?}", output);
}
/*

output: 0	A: 3	Ab: 000011	
output: 6	A: 30	Ab: 011110	
output: 7	A: 15	Ab: 001111	
output: 5	A: 4	Ab: 000100	
output: 1	A: 2	Ab: 000010	
output: 3	A: 0	Ab: 000000	
output: 4	A: 14	Ab: 001110	
output: 2	A: 1	Ab: 000001	

5 = 011
6 = 110

*/

//3 bits => 1 output
//6 bits => 2 output
//9 bits => 3 output
//
//for all 3 bits inc by 1
//    if solution[0] == prog.ins[15-0]
//        for all 6 bits inc by 8
//            if solution[1] == prog.ins[15-1]
            

pub fn _solution2(input: &String) -> isize {
    let prog = Program::from(&input);
    let init_proc = Proc::from(&input);

//    let mut first_vals: FxHashMap<(isize, isize), isize> = FxHashMap::default();
//    let mut done = false;
//    let sought: FxHashSet<(isize, isize, isize, isize, isize, isize, isize, isize)> = FxHashSet::from_iter(vec![
//        (2, 4, 1, 5, 7, 5, 1, 6),
//        (4, 2, 5, 5, 0, 3, 3, 0),
//    ].into_iter());
//    let sought_seq_len = 8;
//    let mut found: FxHashMap<(isize, isize, isize, isize, isize, isize, isize, isize), isize> = FxHashMap::default();
//
//    //_test_a_val(&input, 0b011110);
//    //return 0;
////
//
//    for a_start in 1..8 {
//        let mut total = a_start;
//        for i in 0..16 {
//            total *= 8;
//        }
//        _test_a_val(&input, total);
//    }
//    return 0;
    //


    let mut to_process: VecDeque<(usize, isize, Vec<isize>)> = VecDeque::from([(0, 0, vec![])]);
    let mut results: Vec<isize> = vec![];

    loop {
        let (power, frac_a_val, _) = to_process.pop_front().unwrap();
        let target = prog.instructions[prog.instructions.len() - 1 - power];

        let a_val_start = (frac_a_val >> 3) << 3;

        for a in a_val_start..a_val_start+8 {
            let mut proc = init_proc.clone();
            proc.reg_a = a;
            loop {
                match proc.execute_step(&prog) {
                    Some(result) => {
                        if proc.output == prog.instructions[prog.instructions.len() - proc.output.len()..] {
                            if power < 15 {
                                to_process.push_back((power + 1, a*8, result.clone()));
                                println!("power: {}\t{:?}", power, result);
                            }
                        }
                        if power == 15 {
                            if result == prog.instructions {
                                println!("result found. A: {}", a);
                                println!("{:?}", result);
                                results.push(a);
                            }
                        }
                        break;
                    },
                    None => {
                        if proc.output.len() == 1 {
                            if proc.output[0] != target {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if to_process.is_empty() {
            break;
        }
    }
    
    results.sort();
    println!("{:?}", results);
    results[0]




    /*

    let mut a = 1;
    let mut goal_a = 0;
    for twopow in 0..16 {
        let mut target_found = false;
        let target = prog.instructions[prog.instructions.len() - twopow - 1];
        while !target_found {
            let mut proc = init_proc.clone();
            proc.reg_a = a;
            loop {
                match proc.execute_step(&prog) {
                    Some(result) => {
                        ()
                    },
                    None => {
                        if proc.output.len() == 1 {
                            if proc.output[0] == target {
                                target_found = true;
                                if twopow == 15 {
                                    goal_a = a;
                                }
                                println!("target found A: {}\t target: {}\tpow: {}", a, target, twopow);
                                a *= 8;
                            }
                            break;
                        }
                    }
                }
            }
            a += 1;
        }
    }
    println!("{}", goal_a);
    _test_a_val(&input, goal_a);
    */

    /*
    for twopow in 15..16 {
        println!("POW: {}", twopow);
        let mut a_start = 8_isize.pow(twopow);
        //while a_start % 8 != 6 {
        //    a_start += 1;
        //}
        while a_start % 8 != 3 {
            a_start += 1;
        }
        println!("diff {}", a_start - (1<<15));
        let mut a = a_start;
        let mut found = 0;
        while found < 100000 {
            let mut proc = init_proc.clone();
            //if (a / 8) % 8 != 7 {
            //    a += 8; 
            //    continue;
            //}
            if (a / 64) % 8 != 2 {
                a += (1 << 3); 
                continue;
            }
            proc.reg_a = a;
            loop {
                match proc.execute_step(&prog) {
                    Some(result) => {
                        println!("A:{}\t\t{:?}", a, result);
                        found += 1;
                        a += (1 << 3);
                        break;
                    },
                    None => {
                    }
                }
            }
            //if found.len() == sought.len() {
            //    break;
            //}
        }

    }
    */


    //println!("found {}/{} seq", found.len(), sought.len());
    //for (k, v) in found.iter() {
    //    println!("seq: {:?}\tA: {}\tAb: {}", k, v, v);
    //}

    //let pairs = vec![
    //    (2, 4),
    //    (1, 5),
    //    (7, 5),
    //    (1, 6),
    //    (4, 2),
    //    (5, 5),
    //    (0, 3),
    //];
    //println!("{:#?}", first_vals);
    //for pair in pairs.iter() {
    //    println!("{:?}", pair);
    //    let a = first_vals.get(&pair).unwrap();
    //    println!("{:?}\tA: {}\tAb {}", pair, a, a)
    //}
    //for (k, v) in first_vals.iter() {
    //    println!("output: {}\tA: {}\t Ab: {:b}\t", k, v, v);
    //}
    //0
}

//pub fn _solution2(input: &String) -> isize {
//    let init_proc = Proc::from(&input);
//    let prog = Program::from(&input);
//
//    let mut a_start = 35_184_372_088_832;
//    let chunk_size = 1_000_000_000;
//
//    while a_start < 281_474_976_710_656 {
//        println!("{}B", a_start/1_000_000_000);
//        let mut ranges = vec![];
//        for _i in 0..16 {
//            ranges.push([a_start, a_start + chunk_size]);
//            a_start = a_start + chunk_size;
//        }
//        let mut handles = vec![];
//        for r in ranges {
//            let mut proc = init_proc.clone();
//            let t_prog = prog.clone();
//            let max_steps_without_output = 100;
//            let handle = thread::spawn(move || {
//                let mut output_len;
//                let mut last_output_len;
//                let mut steps_since_output;
//                for a in r[0]..r[1] {
//                    let mut next_a = a;
//                    let mut test_result: isize = 0;
//                    let mut c = false;
//                    for i in 0..16 {
//                        test_result = (((((next_a % 8) ^ 5) ^ 6) ^ ((next_a) / (1 << ((next_a % 8) ^ 5)))) % 8);
//                        if test_result != t_prog.instructions[i] {
//                            c = true;
//                            break;
//                        }
//                        next_a /= 8;
//                    }
//                    if c {
//                        continue;
//                    }
//                    //let expected_steps: Vec<(isize, isize, isize)> = vec![
//                    //    (
//                    //        a,
//                    //        (a % 8),
//                    //        0,
//                    //    ),
//                    //    (
//                    //        a,
//                    //        ((a % 8) ^ 5),
//                    //        0,
//                    //    ),
//                    //    (
//                    //        a,
//                    //        ((a % 8) ^ 5),
//                    //        ((a) / (1 << ((a % 8) ^ 5))),
//                    //    ),
//                    //    (
//                    //        a,
//                    //        (((a % 8) ^ 5) ^ 6),
//                    //        ((a) / (1 << ((a % 8) ^ 5))),
//                    //    ),
//                    //    (
//                    //        a,
//                    //        ((((a % 8) ^ 5) ^ 6) ^ ((a) / (1 << ((a % 8) ^ 5)))),
//                    //        ((a) / (1 << ((a % 8) ^ 5))),
//                    //    ),
//                    //];
//                    //println!("{:#?}", expected_steps);
//                    proc.reg_a = a;
//                    proc.reg_b = init_proc.reg_b;
//                    proc.reg_c = init_proc.reg_c;
//                    proc.ins_ptr = 0;
//                    proc.output.clear();
//                    last_output_len = 0;
//                    steps_since_output = 0;
//                    output_len = 0;
//
//                    let mut num_step = 0;
//                    loop {
//                        num_step += 1;
//                        match proc.execute_step(&t_prog) {
//                            Some(result) => {
//                                if result == t_prog.instructions {
//                                    return Some(a)
//                                } else {
//                                    break;
//                                }
//                            },
//                            None => {
//                                //if num_step == 1 {
//                                //    println!("{:?} {:?}", expected_steps[0], (proc.reg_a, proc.reg_b, proc.reg_c));
//                                //    assert_eq!(expected_steps[0].0, proc.reg_a);
//                                //    assert_eq!(expected_steps[0].1, proc.reg_b);
//                                //    assert_eq!(expected_steps[0].2, proc.reg_c);
//                                //}
//                                //if num_step == 2 {
//                                //    println!("{:?} {:?}", expected_steps[1], (proc.reg_a, proc.reg_b, proc.reg_c));
//                                //    assert_eq!(expected_steps[1].0, proc.reg_a);
//                                //    assert_eq!(expected_steps[1].1, proc.reg_b);
//                                //    assert_eq!(expected_steps[1].2, proc.reg_c);
//                                //}
//                                //if num_step == 3 {
//                                //    println!("{:?} {:?}", expected_steps[2], (proc.reg_a, proc.reg_b, proc.reg_c));
//                                //    assert_eq!(expected_steps[2].0, proc.reg_a);
//                                //    assert_eq!(expected_steps[2].1, proc.reg_b);
//                                //    assert_eq!(expected_steps[2].2, proc.reg_c);
//                                //}
//                                //if num_step == 4 {
//                                //    println!("{:?} {:?}", expected_steps[3], (proc.reg_a, proc.reg_b, proc.reg_c));
//                                //    assert_eq!(expected_steps[3].0, proc.reg_a);
//                                //    assert_eq!(expected_steps[3].1, proc.reg_b);
//                                //    assert_eq!(expected_steps[3].2, proc.reg_c);
//                                //}
//                                //if num_step == 5 {
//                                //    println!("{:?} {:?}", expected_steps[4], (proc.reg_a, proc.reg_b, proc.reg_c));
//                                //    assert_eq!(expected_steps[4].0, proc.reg_a);
//                                //    assert_eq!(expected_steps[4].1, proc.reg_b);
//                                //    assert_eq!(expected_steps[4].2, proc.reg_c);
//                                //}
//                                //if proc.output.len() > t_prog.instructions.len() {
//                                //    break;
//                                //} else if last_output_len != proc.output.len() {
//                                if last_output_len != proc.output.len() {
//                                    for i in last_output_len..proc.output.len() {
//                                        if t_prog.instructions[i] != proc.output[i] {
//                                            break;
//                                        }
//                                    }
//                                    last_output_len = output_len;
//                                    steps_since_output = 0;
//                                } else {
//                                    steps_since_output += 1;
//                                    if steps_since_output == max_steps_without_output {
//                                        break;
//                                    }
//                                }
//                            }
//                        }
//                    }
//                }
//                None
//            });
//            handles.push(handle);
//        }
//        let results: Vec<Option<isize>> = handles.into_iter().map(|h| h.join().unwrap()).collect();
//        for r in results.iter() {
//            match r {
//                Some(a) => return *a,
//                None => (),
//            }
//        }
//    }
//
//    panic!("no solution found");
//}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn test_example_day_17_1_1() {
        let path = common::get_test_data_path("day17/case1.txt").unwrap();
        let result = solution1(&path);
        assert_eq!(result, String::from("4,6,3,5,6,3,5,2,1,0"))
    }

    #[test]
    fn test_example_day_17_1_2() {
        let mut proc = Proc::new(0, 0, 9);
        let prog = Program {
            instructions: vec![2, 6],
        };
        let _result = proc.execute(&prog);
        assert_eq!(proc.reg_b, 1);
    }

    #[test]
    fn test_example_day_17_1_3() {
        let mut proc = Proc::new(10, 0, 0);
        let prog = Program {
            instructions: vec![5, 0, 5, 1, 5, 4],
        };
        let result = proc.execute(&prog);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_example_day_17_1_4() {
        let mut proc = Proc::new(2024, 0, 0);
        let prog = Program {
            instructions: vec![0, 1, 5, 4, 3, 0],
        };
        let result = proc.execute(&prog);
        assert_eq!(result, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(proc.reg_a, 0);
    }

    #[test]
    fn test_example_day_17_1_5() {
        let mut proc = Proc::new(0, 29, 0);
        let prog = Program {
            instructions: vec![1, 7],
        };
        let _result = proc.execute(&prog);
        assert_eq!(proc.reg_b, 26);
    }

    #[test]
    fn test_example_day_17_1_6() {
        let mut proc = Proc::new(0, 2024, 43690);
        let prog = Program {
            instructions: vec![4, 0],
        };
        let _result = proc.execute(&prog);
        assert_eq!(proc.reg_b, 44354);
    }

    #[test]
    fn test_example_day_17_2() {
        let path = common::get_test_data_path("day17/case2.txt").unwrap();
        let result = solution2(&path);
        assert_eq!(result, 117440);
    }
}
