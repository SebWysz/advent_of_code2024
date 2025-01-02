use std::collections::{HashMap, HashSet};
use std::{fs::File, io, io::BufRead};

pub fn first() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day5.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut print_before: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut print_after: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Assemble hashmap rules
    lines
        .by_ref()
        .take_while(|x| x.as_ref().expect("Reader Error") != "")
        .for_each(|x| {
            x.unwrap()
                .split_once("|")
                .into_iter()
                .for_each(|(left, right)| {
                    let left = left.parse::<i32>().unwrap();
                    let right = right.parse::<i32>().unwrap();
                    print_before
                        .entry(left)
                        .and_modify(|x| {
                            x.insert(right);
                        })
                        .or_default();
                    print_after
                        .entry(right)
                        .and_modify(|x| {
                            x.insert(left);
                        })
                        .or_default();
                })
        });

    let default = HashSet::new();
    let result = lines
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|nums| {
            let n = nums.len();
            let initial = nums[n / 2];

            if (1..n).any(|i| {
                (0..i).any(|j| {
                    print_before
                        .get(&nums[i])
                        .unwrap_or(&default)
                        .contains(&nums[j])
                })
            }) {
                0
            } else {
                initial
            }
        })
        .sum();

    Ok(result)
}
