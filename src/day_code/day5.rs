use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs::File, io, io::BufRead, io::BufReader, io::Lines};

fn assemble_rules_hashmap(lines: &mut Lines<BufReader<File>>) -> HashMap<i32, HashSet<i32>> {
    let mut print_before: HashMap<i32, HashSet<i32>> = HashMap::new();
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
                        .or_insert_with(HashSet::new)
                        .insert(right);
                })
        });

    print_before
}

pub fn first() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day5.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let print_before = assemble_rules_hashmap(&mut lines);

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

fn topological_sort(nums: &[i32], print_before: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    // Build graph and calculate in-degrees
    for &num in nums {
        graph.entry(num).or_default();
        in_degree.entry(num).or_insert(0);
    }

    for &num in nums {
        if let Some(deps) = print_before.get(&num) {
            for &dep in deps {
                if nums.contains(&dep) {
                    graph.entry(dep).or_default().push(num);
                    *in_degree.entry(num).or_default() += 1;
                }
            }
        }
    }

    // Start with nodes that have no dependencies
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&num, _)| num)
        .collect();

    // Process queue
    while let Some(num) = queue.pop_front() {
        result.push(num);

        if let Some(deps) = graph.get(&num) {
            for &next in deps {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

pub fn second() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day5.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let print_before = assemble_rules_hashmap(&mut lines);

    let default = HashSet::new();
    let result = lines
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| {
            (1..nums.len()).any(|i| {
                (0..i).any(|j| {
                    print_before
                        .get(&nums[i])
                        .unwrap_or(&default)
                        .contains(&nums[j])
                })
            })
        })
        .map(|nums| {
            let mut sorted = topological_sort(&nums, &print_before);
            sorted.reverse();
            sorted[sorted.len() / 2]
        })
        .sum();

    Ok(result)
}
