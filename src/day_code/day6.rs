use std::collections::HashMap;
use std::{fs::File, io, io::BufRead};

pub fn first() -> Result<i32, std::io::Error> {
    let file = File::open("src/textfiles/day6.txt")?;
    let mut field: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut turn90: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    turn90.insert((1, 0), (0, -1));
    turn90.insert((0, 1), (1, 0));
    turn90.insert((-1, 0), (0, 1));
    turn90.insert((0, -1), (-1, 0));

    let num_rows = field.len();
    let num_cols = field[0].len();
    let mut location = (-1, -1);
    for row in 0..num_rows {
        for col in 0..num_cols {
            if field[row][col] == '^' {
                location = (row as i32, col as i32);
                field[row][col] = 'X';
                break;
            }
        }
        if location != (-1, -1) {
            break;
        }
    }

    println!("{:?}", location);
    let mut buffer = 1000;
    let mut dir = (-1, 0);
    location.0 += dir.0;
    location.1 += dir.1;
    field[location.0 as usize][location.1 as usize] = 'X';
    while buffer > 0 {
        let next = (location.0 + dir.0, location.1 + dir.1);
        if next.0 < 0 || next.0 >= num_rows as i32 || next.1 < 0 || next.1 >= num_cols as i32 {
            break;
        } else if field[next.0 as usize][next.1 as usize] == '#' {
            dir = turn90[&dir].clone();
        } else {
            location = next.clone();
            if field[next.0 as usize][next.1 as usize] == 'X' {
                buffer -= 1;
            } else {
                field[next.0 as usize][next.1 as usize] = 'X';
                buffer = 1000;
            }
        }
    }
    field.iter().for_each(|row| {
        row.iter().for_each(|x| print!("{}", x));
        print!("\n");
    });
    let result = field.iter().flatten().filter(|&x| *x == 'X').count();

    Ok(result as i32)
}
