use regex::Regex;
use std::{fs::File, io, io::BufRead};

pub fn first() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day3.txt")?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mul_result = 0;
    for line in reader.lines() {
        let line = line?;
        for capture in re.captures_iter(&line) {
            let num1: i32 = capture[1].parse().unwrap();
            let num2: i32 = capture[2].parse().unwrap();
            mul_result += num1 * num2;
        }
    }

    Ok(mul_result)
}

pub fn second() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day3.txt")?;
    let reader = io::BufReader::new(file);
    // Closure starts     1     2     3        4        5
    // Closure ends                 2     3  1        4            5
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don\'t\(\))").unwrap();

    let mut mul_result = 0;
    let mut execute = true;
    for line in reader.lines() {
        let line = line?;
        for capture in re.captures_iter(&line) {
            if let Some(_mul_match) = capture.get(1) {
                if execute {
                    let num1: i32 = capture[2].parse().unwrap();
                    let num2: i32 = capture[3].parse().unwrap();
                    mul_result += num1 * num2;
                }
            } else if let Some(_do_match) = capture.get(4) {
                execute = true;
            } else if let Some(_dont_match) = capture.get(5) {
                execute = false;
            }
        }
    }

    Ok(mul_result)
}
