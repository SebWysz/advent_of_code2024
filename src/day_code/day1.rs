use std::collections::HashMap;
use std::{fs::File, io, io::BufRead};

fn read_text_file() -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let file = File::open("src/textfiles/day1.txt")?;
    let reader = io::BufReader::new(file);

    let mut first_vec: Vec<i32> = vec![];
    let mut second_vec: Vec<i32> = vec![];
    first_vec.reserve(1000);
    second_vec.reserve(1000);
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter(|x| *x != "")
            .map(|x| x.parse::<i32>().expect("Not numbers"))
            .collect();
        first_vec.push(nums[0]);
        second_vec.push(nums[1]);
    }
    Ok((first_vec, second_vec))
}

pub fn first() -> Result<i32, std::io::Error> {
    let (mut first_vec, mut second_vec) = read_text_file()?;
    first_vec.sort();
    second_vec.sort();
    let distance = first_vec
        .iter()
        .zip(second_vec.iter())
        .map(|(x, y)| (x - y).abs())
        .reduce(|acc, x| acc + x);

    Ok(distance.expect("No Distance"))
}

pub fn second() -> Result<i32, std::io::Error> {
    let (first_vec, second_vec) = read_text_file()?;

    let mut map: HashMap<&i32, i32> = HashMap::new();

    for num in second_vec.iter() {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let sim_score = first_vec
        .iter()
        .map(|x| x * map.get(x).unwrap_or(&0))
        .reduce(|acc, x| acc + x);

    Ok(sim_score.expect("Reduce Failed"))
}
