use std::fs;
use std::path::PathBuf;

use rustc_hash::FxHashMap;

pub mod constants {
    pub const INPUT_PATH: &str = "day22/input.txt";
}

pub fn solution1(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    let mut init_secrets: Vec<isize> = vec![];

    for line in input.lines() {
        init_secrets.push(line.parse::<isize>().unwrap());
    }
    _solution1(&init_secrets)
}

pub fn get_secret(init: isize, iterations: isize) -> isize {
    let mut secret = init;
    for _i in 0..iterations {
        let a = secret * 64;

        secret = secret ^ a;
        secret = secret % 16777216;

        let b = secret / 32;

        secret = secret ^ b;
        secret = secret % 16777216;

        let c = secret * 2048;

        secret = secret ^ c;
        secret = secret % 16777216;
    }
    secret
}

pub fn get_all_secrets(init: isize, iterations: isize) -> Vec<isize> {
    let mut result = vec![init];
    result.reserve(usize::try_from(iterations).unwrap());

    let mut secret = init;
    for _i in 0..iterations {
        let a = secret * 64;

        secret = secret ^ a;
        secret = secret % 16777216;

        let b = secret / 32;

        secret = secret ^ b;
        secret = secret % 16777216;

        let c = secret * 2048;

        secret = secret ^ c;
        secret = secret % 16777216;

        result.push(secret);
    }
    result
}

pub fn secrets_to_prices(secrets: &Vec<isize>) -> Vec<isize> {
    secrets.iter().map(|i| i % 10).collect()
}

pub fn price_diffs(prices: &Vec<isize>) -> Vec<isize> {
    let mut result = vec![];
    result.reserve(prices.len());

    for i in 1..prices.len() {
        result.push(prices[i] - prices[i-1]);
    }

    result
}

pub fn _solution1(input: &Vec<isize>) -> isize {
    let mut result = 0;
    for s in input.iter() {
        result += dbg!(get_secret(*s, 2000));
    }
    result
}

pub fn solution2(path: &PathBuf) -> isize {
    let input =  fs::read_to_string(path).unwrap();
    let mut init_secrets: Vec<isize> = vec![];

    for line in input.lines() {
        init_secrets.push(line.parse::<isize>().unwrap());
    }
    _solution2(&init_secrets)
}

pub fn _solution2(init_secrets: &Vec<isize>) -> isize {
    let mut totals: FxHashMap<(isize, isize, isize, isize), isize> = FxHashMap::default();
    totals.reserve(20*20*20*20);

    for init_secret in init_secrets.iter() {
        let mut secret_totals: FxHashMap<(isize, isize, isize, isize), isize> = FxHashMap::default();
        let secrets = get_all_secrets(*init_secret, 2000);
        let prices = secrets_to_prices(&secrets);
        let price_diffs = price_diffs(&prices);
        for i in 3..price_diffs.len() {
            let k = (price_diffs[i-3], price_diffs[i-2], price_diffs[i-1], price_diffs[i]);
            if !secret_totals.contains_key(&k) {
                secret_totals.entry(k.clone()).or_insert(prices[i+1]);
                if !totals.contains_key(&k) {
                    totals.entry(k.clone()).or_insert(prices[i+1]);
                } else {
                    totals.entry(k).and_modify(|v| *v += prices[i+1]);
                }
            }
        }
    }
    let mut max_bananas = 0;
    let mut best_seq = (0, 0, 0 , 0);
    for (seq, val) in totals.iter() {
        if *val > max_bananas {
            max_bananas = *val;
            best_seq = *seq;
        }
    }
    println!("{:?}", best_seq);
    max_bananas
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn day_22_1_1() {
        let path = common::get_test_data_path("day22/case1.txt").unwrap();
        assert_eq!(solution1(&path), 37327623);
    }

    #[test]
    fn day_22_2_1() {
        let path = common::get_test_data_path("day22/case2.txt").unwrap();
        assert_eq!(solution2(&path), 23);
    }
}
