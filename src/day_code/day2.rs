use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(PartialEq, Eq)]
enum IncDec {
    Increasing,
    Decreasing,
    Invalid,
}

fn small_and_consistent_check(incdec: &IncDec, num1: &i32, num2: &i32) -> bool {
    match num2 - num1 {
        0 => false,
        x if x.abs() > 3 => false,
        x if x > 0 => *incdec == IncDec::Increasing,
        x if x < 0 => *incdec == IncDec::Decreasing,
        _ => panic!("Number diff not covered by match?"),
    }
}

pub fn first() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day2.txt")?;
    let reader = io::BufReader::new(file);

    let mut num_safe = 0;
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap_or(-10)).collect();

        // if nums[1] - nums[0] == 0 then small_and_consistent_check will catch
        let mut incdec = if nums[1] - nums[0] > 0 {
            IncDec::Increasing
        } else {
            IncDec::Decreasing
        };

        for window in nums.windows(2) {
            if let [prev, curr] = window {
                if !small_and_consistent_check(&incdec, prev, curr) {
                    incdec = IncDec::Invalid;
                    break;
                }
            }
        }
        if incdec != IncDec::Invalid {
            num_safe += 1;
        }
    }

    Ok(num_safe)
}

#[derive(PartialEq, Eq)]
enum Tolerance {
    // i32 is prev prev and should
    // be updated in each iteration
    Good(i32),
    NotSoGood,
    Bad,
}

pub fn second() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day2.txt")?;
    let reader = io::BufReader::new(file);

    let mut num_safe = 0;
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap_or(-10)).collect();

        // if nums[1] - nums[0] == 0 then small_and_consistent_check will catch
        let incdec = if nums[1] - nums[0] > 0 {
            IncDec::Increasing
        } else {
            IncDec::Decreasing
        };
        let mut tol = Tolerance::Good(-1);

        for window in nums.windows(2) {
            if let [prev, curr] = window {
                if tol != Tolerance::Bad && !small_and_consistent_check(&incdec, prev, curr) {
                    tol = match tol {
                        Tolerance::Good(x) => {
                            if x == -1 {
                                Tolerance::NotSoGood
                            } else if small_and_consistent_check(&incdec, &x, curr) {
                                Tolerance::NotSoGood
                            } else {
                                Tolerance::Bad
                            }
                        }
                        _ => Tolerance::Bad,
                    };
                } else if tol == Tolerance::Bad {
                    break;
                }
            }
        }
        if tol != Tolerance::Bad {
            num_safe += 1;
        }
    }

    Ok(num_safe)
}
