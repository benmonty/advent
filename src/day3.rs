use std::error::Error;
use std::path::PathBuf;
use std::fs;
use regex::Regex;

pub mod constants {
    pub const MEMORY_FILE_PATH: &str = "day3/memory.txt";
}


pub fn multiply(memory_file: PathBuf) -> Result<u64, Box<dyn Error>> {
    let memory = fs::read_to_string(memory_file)?;
    Ok(multiply_str(&memory))
}

pub fn multiply_str(memory: &String) -> u64 {
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("invalid regex");
    let mut operands = vec![];
    let mut count = 0;
    let mut result = 0;
    for (_, [num1, num2]) in rx.captures_iter(&memory).map(|c| c.extract()) {
        let num1 = num1.parse::<u64>().unwrap();
        let num2 = num2.parse::<u64>().unwrap();
        count += 1;
        operands.push((num1, num2));
        println!("{} mul({}, {})", count, num1, num2);
        result += num1*num2;
    }
    result
}

enum MultMode {
    ENABLED,
    DISABLED,
}

pub fn cond_multiply(memory_file: PathBuf) -> Result<u64, Box<dyn Error>> {
    let memory = fs::read_to_string(memory_file)?;
    Ok(cond_multiply_str(&memory))
}

pub fn cond_multiply_str(memory: &String) -> u64 {
    const ENABLE_MUL_TOKEN: &str = "do()";
    const DISABLE_MUL_TOKEN: &str = "don't()";

    let rx = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").expect("invalid regex");
    let mut mode = MultMode::ENABLED;
    let mut result = 0;

    for capture in rx.captures_iter(&memory) {
        let full_match = &capture[0];
        if full_match == ENABLE_MUL_TOKEN {
            mode = MultMode::ENABLED;
        } else if full_match == DISABLE_MUL_TOKEN {
            mode = MultMode::DISABLED;
        } else {
            match mode {
                MultMode::ENABLED => {
                    let num1 = capture[1].parse::<u64>().unwrap();
                    let num2 = capture[2].parse::<u64>().unwrap();
                    result += num1 * num2;
                },
                MultMode::DISABLED => {
                   // do nothing 
                },
            };
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let s = String::from(r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(multiply_str(&s), 161, "correctly computes example result");
    }

    #[test]
    fn test_conditional_example() {
        let s = String::from(r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(cond_multiply_str(&s), 48, "correctly computes example result");
    }
}
