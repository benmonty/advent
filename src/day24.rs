use std::fs;
use std::path::PathBuf;

use rustc_hash::{FxHashMap, FxHashSet};
use rand::Rng;

pub mod constants {
    pub const INPUT_PATH: &str = "day24/input.txt";
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}

pub struct Device {
    wires: Vec<Wire>,
    gates: Vec<Gate>,
    output_vals: FxHashMap<String, isize>,
}

impl Device {
    pub fn seed_wire_outputs(&mut self) {
        for w in self.wires.iter() {
            self.output_vals.entry(w.id.clone()).or_insert(w.output_val);
        }
    }

    pub fn from_sum(gates: Vec<Gate>, x: isize, y: isize) -> Self {
        if x > (1 << 44) || y > (1 << 44) {
            panic!("x or y too large");
        }
        let mut wires = vec![];

        for i in 0..45 {
            let output = match ((1 << i) & x) > 0 {
                true => 1,
                false => 0,
            };
            wires.push(Wire {
                id: format!("x{:02}", i),
                output_val: output,
            })
        }
        for i in 0..45 {
            let output = match ((1 << i) & y) > 0 {
                true => 1,
false => 0,
            };
            wires.push(Wire {
                id: format!("y{:02}", i),
                output_val: output,
            })
        }
        Self {
            wires,
            gates,
            output_vals: FxHashMap::default(),
        }
    }
}


#[derive(Debug)]
pub struct Wire {
    id: String,
    output_val: isize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Gate {
    input1: String,
    input2: String,
    output: String,
    kind: GateKind,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum GateKind {
    XOR,
    OR,
    AND,
}

pub fn parse(input: &String) -> Device {
    let wire_lines = input.lines().take_while(|&l| l != "");
    let gate_lines = input.lines().rev().take_while(|&l| l != "");

    let mut wires = vec![];
    for wl in wire_lines {
        let (id, output_val) = wl.split_once(": ").unwrap();
        wires.push(Wire {
            id: id.to_string(),
            output_val: output_val.parse::<isize>().unwrap(),
        });
    }

    let mut gates = vec![];
    for gl in gate_lines {
        let mut components = gl.split_whitespace();
        let input1 = components.next().unwrap().to_string();
        let op = components.next().unwrap();
        let input2 = components.next().unwrap().to_string();
        let _arrow = components.next().unwrap();
        let output = components.next().unwrap().to_string();
        let gatekind = match op {
            "XOR" => GateKind::XOR,
            "OR" => GateKind::OR,
            "AND" => GateKind::AND,
            _ => panic!("unexpected gatekind"),
        };
        gates.push(Gate {
            input1,
            input2,
            output,
            kind: gatekind,
        })
    }

    Device {
        wires,
        gates,
        output_vals: FxHashMap::default(),
    }
}

pub fn compute_z_output(device: &mut Device) -> isize {
    device.seed_wire_outputs();

    let mut new_outputs: FxHashSet<String> = device.output_vals.keys().cloned().collect();
    let max = 100000;
    let mut ct = 0;
    while new_outputs.len() > 0 {
        ct += 1;
        if ct > max {
            return 99999;
        }
        let mut next_new_outputs: FxHashSet<String> = FxHashSet::default();

        for gate in device.gates.iter() {
            if new_outputs.contains(&gate.input1) || new_outputs.contains(&gate.input2) {
                let input1_o = device.output_vals.get(&gate.input1);
                let input2_o = device.output_vals.get(&gate.input2);
                match (input1_o, input2_o) {
                    (Some(input1_o), Some(input2_o)) => {
                        let new_output = match gate.kind {
                            GateKind::OR => input1_o | input2_o,
                            GateKind::XOR => input1_o ^ input2_o,
                            GateKind::AND => input1_o & input2_o,
                        };
                        device.output_vals.entry(gate.output.clone()).or_insert(new_output);
                        next_new_outputs.insert(gate.output.clone());
                    }
                    _ => (),
                }
            }
        }
        new_outputs = next_new_outputs;
    }
    let mut z_count = 0;
    let mut result = 0;
    while device.output_vals.contains_key(&format!("z{:02}", z_count)) {
        let mut output = *device.output_vals.get(&format!("z{:02}", z_count)).unwrap();
        output <<= z_count;
        result ^= output;
        z_count += 1;
    }
    result
}

pub fn _solution1(input: &String) -> isize {
    let mut device = parse(&input);
    compute_z_output(&mut device)
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

//x08 y08 -> z08
//x14 y14 -> z14
//x22 y22 -> z22
//x29 y29 -> z29
//
//x07 y07 -> z08
//x14 y14 -> z15
//x21 y21 -> z22
//x22 y22 -> z23
//x28 y28 -> z29
//x29 y29 -> z30

//(7)
//8
//14
//(21)
//22
//(28)
//29
//
//
fn bit_at(num: isize, idx: isize) -> isize {
    (num >> idx) & 1
}
fn render_as_zs(num: isize) {
    for i in 0..45 {
        let bit = bit_at(num, i);
        if bit == 1 {
            println!("\tz{:02} = {}", i, bit);
        }
    }
}

fn rand_test(device: &Device, gate_overrides: &Vec<Gate>, iterations: isize) -> isize {
    let mut fail_count = 0;
    let mut rng = rand::thread_rng();
    for _i in 0..iterations {
        let x = rng.gen_range(0..=1<<44);
        let y = rng.gen_range(0..=1<<44);
        let mut d = Device::from_sum(gate_overrides.clone(), x, y);
        d.seed_wire_outputs();
        let output = compute_z_output(&mut d);
        //println!("x{:02} + y{:02}  == (z{:02} = 0), z{:02} == 1", i, i, i, i + 1);
        if output != x+y {
            fail_count += 1;
        }
    }
    fail_count
}

fn search_for_corruption(device: &Device) {
    let mut total_fails = 0;
    for i in 0..45 {
        println!("====== i = {} =======", i);
        {
            let x = 1 << i;
            let y = 1 << i;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 0), z{:02} == 1", i, i, i, i + 1);
            if output == 2*(1 << i) {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
        {
            let x = 1 << i;
            let y = 0;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 1)", i, i, i);
            if output == 1 << i {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
        {
            let x = 0;
            let y = 1 << i;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 1)", i, i, i);
            if output == 1 << i {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
    }
}

fn get_potential_swaps(device: &Device, shifts: &Vec<isize>, tainted_inputs: &Vec<String>) -> Vec<(Gate, Gate)> {
    let mut total_fails = 0;
    for i in shifts.iter() {
        {
            let x = 1 << i;
            let y = 1 << i;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 0), z{:02} == 1", i, i, i, i + 1);
            if output == 2*(1 << i) {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
        {
            let x = 1 << i;
            let y = 0;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 1)", i, i, i);
            if output == 1 << i {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
        {
            let x = 0;
            let y = 1 << i;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 1)", i, i, i);
            if output == 1 << i {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
        {
            let x = 0;
            let y = 0;
            let mut d = Device::from_sum(device.gates.clone(), x, y);
            d.seed_wire_outputs();
            let output = compute_z_output(&mut d);
            println!("x{:02} + y{:02}  == (z{:02} = 0)", i, i, i);
            if output == 0 {
            } else {
                println!("\tFAIL");
                render_as_zs(output);
                total_fails += 1;
            }
        }
    }

    let mut tainted = tainted_inputs.clone();
    let mut maybe_corrupt: FxHashSet<Gate> = FxHashSet::default();
    while tainted.len() > 0 {
        let mut new_tainted = vec![];
        for tainted_input in tainted.iter() {
            for g in device.gates.iter() {
                if g.input1 == *tainted_input || g.input2 == *tainted_input {
                    if maybe_corrupt.insert(g.clone()) {
                        new_tainted.push(g.output.clone());
                    }
                }
                if g.output == *tainted_input {
                    if maybe_corrupt.insert(g.clone()) {
                        if !g.input1.starts_with("x") && !g.input1.starts_with("y") {
                            new_tainted.push(g.input1.clone());
                        }
                        if !g.input2.starts_with("x") && !g.input2.starts_with("y") {
                            new_tainted.push(g.input2.clone());
                        }
                    }
                }
            }
        }
        tainted = new_tainted;
    }
    println!("num maybe corrupt: {}", maybe_corrupt.len());

    let maybe_corrupt: Vec<Gate> = maybe_corrupt.into_iter().collect();
    let mut potential_swaps = vec![];
    for i in 0..maybe_corrupt.len()-1 {
        for j in i+1..maybe_corrupt.len() {
            potential_swaps.push((maybe_corrupt[i].clone(), maybe_corrupt[j].clone()));
        }
    }

    let mut gate_map: FxHashMap<(String, String), FxHashMap<GateKind, usize>> = FxHashMap::default();
    for (i, g) in device.gates.iter().enumerate() {
        gate_map.entry((g.input1.clone(), g.input2.clone())).or_default();
        gate_map.get_mut(&(g.input1.clone(), g.input2.clone())).unwrap().entry(g.kind.clone()).or_insert(i);
    }

    let mut results = vec![];
    for i in 0..potential_swaps.len() {
        let s1 = &potential_swaps[i];
        let mut swapped_gates = device.gates.clone();


        let s1_g1_idx = gate_map.get(&(s1.0.input1.clone(), s1.0.input2.clone())).unwrap().get(&s1.0.kind).unwrap();
        let s1_g2_idx = gate_map.get(&(s1.1.input1.clone(), s1.1.input2.clone())).unwrap().get(&s1.1.kind).unwrap();
        swapped_gates[*s1_g1_idx].output = device.gates[*s1_g2_idx].output.clone();
        swapped_gates[*s1_g2_idx].output = device.gates[*s1_g1_idx].output.clone();

        let mut fails = 0;
        for i in shifts.iter() {
            {
                let x = 1 << i;
                let y = 0;
                let mut d = Device::from_sum(swapped_gates.clone(), x, y);
                d.seed_wire_outputs();
                let output = compute_z_output(&mut d);
                //println!("{}th bit X[{}] == output[{}]", i, x, output);
                if output == 1 << i {
                } else {
                    fails += 1;
                    //println!("\tFAIL");
                }
            }
            {
                let x = 0;
                let y = 1 << i;
                let mut d = Device::from_sum(swapped_gates.clone(), x, y);
                d.seed_wire_outputs();
                let output = compute_z_output(&mut d);
                //println!("{}th bit X[{}] == output[{}]", i, x, output);
                if output == 1 << i {
                } else {
                    fails += 1;
                    //println!("\tFAIL");
                }
            }
            {
                let x = 1 << i;
                let y = 1 << i;
                let mut d = Device::from_sum(swapped_gates.clone(), x, y);
                d.seed_wire_outputs();
                let output = compute_z_output(&mut d);
                //println!("{}th bit X[{}] == output[{}]", i, x, output);
                if output == 2*(1 << i) {
                } else {
                    fails += 1;
                    //println!("\tFAIL");
                }
            }
            {
                let x = 0;
                let y = 0;
                let mut d = Device::from_sum(swapped_gates.clone(), x, y);
                d.seed_wire_outputs();
                let output = compute_z_output(&mut d);
                //println!("{}th bit X[{}] == output[{}]", i, x, output);
                if output == 0 {
                } else {
                    fails += 1;
                    //println!("\tFAIL");
                }
            }
        }
        if fails == 0 {
            results.push((s1.0.clone(), s1.1.clone()));
        }
    }
    results
}

pub fn _solution2(input: &String) -> usize {
    let device = parse(&input);
    search_for_corruption(&device);

    let test_groups = vec![
        (
            vec![6, 7, 8, 9],
            vec![
                String::from("x07"), String::from("y07"),
                String::from("x08"), String::from("y08"),
                String::from("x09"), String::from("y09"),

                String::from("z09")
            ] ,
        ),
        (
            vec![13, 14, 15, 16, 17],
            vec![
                String::from("x14"), String::from("y14"),
                String::from("x15"), String::from("y15"),
                String::from("x16"), String::from("y16"),

                String::from("z15"), String::from("z14"),
            ] ,
        ),
        (
            vec![19, 20, 21, 22, 23, 24],
            vec![
                String::from("x21"), String::from("y21"),
                String::from("x22"), String::from("y22"),

                String::from("z22"), String::from("z23"),
            ] ,
        ),
        (
            vec![28, 29, 30],
            vec![
                String::from("x30"), String::from("y30"),
                String::from("x29"), String::from("y29"),
                String::from("x28"), String::from("y28"),

                String::from("z29"), String::from("z30"),
                String::from("z28")

            ],
        ),
    ];

    let mut potential_swaps = vec![];

    let mut found = true;
    for tg in test_groups.iter() {
        let ps = get_potential_swaps(&device, &tg.0, &tg.1);
        potential_swaps.push(ps);
    }
    for ps in potential_swaps.iter() {
        if ps.len() == 0 {
            found = false;
        }
    }
    println!("potential swaps");
    println!("{:#?}", potential_swaps);
    if !found {
        panic!("failed to find possible swaps in each group");
    }
    let g1 = &potential_swaps[0];
    let g2 = &potential_swaps[1];
    let g3 = &potential_swaps[2];
    let g4 = &potential_swaps[3];

    let mut gate_map: FxHashMap<(String, String), FxHashMap<GateKind, usize>> = FxHashMap::default();
    for (i, g) in device.gates.iter().enumerate() {
        gate_map.entry((g.input1.clone(), g.input2.clone())).or_default();
        gate_map.get_mut(&(g.input1.clone(), g.input2.clone())).unwrap().entry(g.kind.clone()).or_insert(i);
    }

    for s1 in g1.iter() {
        for s2 in g2.iter() {
            for s3 in g3.iter() {
                for s4 in g4.iter() {
                    let mut swapped_gates = device.gates.clone();

                    let s1_g1_idx = gate_map.get(&(s1.0.input1.clone(), s1.0.input2.clone())).unwrap().get(&s1.0.kind).unwrap();
                    let s1_g2_idx = gate_map.get(&(s1.1.input1.clone(), s1.1.input2.clone())).unwrap().get(&s1.1.kind).unwrap();
                    swapped_gates[*s1_g1_idx].output = device.gates[*s1_g2_idx].output.clone();
                    swapped_gates[*s1_g2_idx].output = device.gates[*s1_g1_idx].output.clone();

                    let s2_g1_idx = gate_map.get(&(s2.0.input1.clone(), s2.0.input2.clone())).unwrap().get(&s2.0.kind).unwrap();
                    let s2_g2_idx = gate_map.get(&(s2.1.input1.clone(), s2.1.input2.clone())).unwrap().get(&s2.1.kind).unwrap();
                    swapped_gates[*s2_g1_idx].output = device.gates[*s2_g2_idx].output.clone();
                    swapped_gates[*s2_g2_idx].output = device.gates[*s2_g1_idx].output.clone();

                    let s3_g1_idx = gate_map.get(&(s3.0.input1.clone(), s3.0.input2.clone())).unwrap().get(&s3.0.kind).unwrap();
                    let s3_g2_idx = gate_map.get(&(s3.1.input1.clone(), s3.1.input2.clone())).unwrap().get(&s3.1.kind).unwrap();
                    swapped_gates[*s3_g1_idx].output = device.gates[*s3_g2_idx].output.clone();
                    swapped_gates[*s3_g2_idx].output = device.gates[*s3_g1_idx].output.clone();

                    let s4_g1_idx = gate_map.get(&(s4.0.input1.clone(), s4.0.input2.clone())).unwrap().get(&s4.0.kind).unwrap();
                    let s4_g2_idx = gate_map.get(&(s4.1.input1.clone(), s4.1.input2.clone())).unwrap().get(&s4.1.kind).unwrap();
                    swapped_gates[*s4_g1_idx].output = device.gates[*s4_g2_idx].output.clone();
                    swapped_gates[*s4_g2_idx].output = device.gates[*s4_g1_idx].output.clone();

                    let fail_count = rand_test(&device, &swapped_gates, 10000);
                    if fail_count == 0 {
                        println!("found solution");
                        let mut v = vec![
                            swapped_gates[*s1_g1_idx].output.clone(),
                            swapped_gates[*s1_g2_idx].output.clone(),
                            swapped_gates[*s2_g1_idx].output.clone(),
                            swapped_gates[*s2_g2_idx].output.clone(),
                            swapped_gates[*s3_g1_idx].output.clone(),
                            swapped_gates[*s3_g2_idx].output.clone(),
                            swapped_gates[*s4_g1_idx].output.clone(),
                            swapped_gates[*s4_g2_idx].output.clone(),
                        ];
                        v.sort();
                        println!("{}", v.join(","));
                    } else {
                        println!("failed {}", fail_count);
                        println!("{:#?}", s1);
                        println!("{:#?}", s2);
                        println!("{:#?}", s3);
                        println!("{:#?}", s4);
                    }
                }
            }
        }
    }

    0

}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn day24_1_1() {
        let path = common::get_test_data_path("day24/case1.txt").unwrap();
        assert_eq!(solution1(&path), 2024);
    }

    #[test]
    fn day24_2_1() {
        assert!(false, "todo")
    }
}

//z08
//thm
//wrm
//wss
//z22
//fjs|hwq
//gbs
//grd|z29
//
//fjs,gbs,grd,thm,wrm,wss,z08,z22
//
//fjs,gbs,thm,wrm,wss,z08,z22,z29
//
//gbs,grd,hwq,thm,wrm,wss,z08,z22
//
//gbs,hwq,thm,wrm,wss,z08,z22,z29
