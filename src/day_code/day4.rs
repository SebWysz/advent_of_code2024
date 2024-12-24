use std::fs;

fn parse_to_grid() -> Result<Vec<Vec<char>>, std::io::Error> {
    let string = fs::read_to_string("src/textfiles/day4.txt")?;
    Ok(string.lines().map(|line| line.chars().collect()).collect())
}

pub fn first() -> Result<i32, std::io::Error> {
    let grid = parse_to_grid()?;
    let rows = grid.len();
    let cols = grid[0].len();
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let target_str = "XMAS";
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut valid = true;
                for (i, target_char) in target_str.chars().enumerate() {
                    let new_row = row as i32 + dx * i as i32;
                    let new_col = col as i32 + dy * i as i32;

                    if new_row < 0
                        || new_row >= rows as i32
                        || new_col < 0
                        || new_col >= cols as i32
                        || grid[new_row as usize][new_col as usize] != target_char
                    {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn check_corners(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let first = (grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
        || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M');
    let second = (grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
        || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M');

    first && second
}

pub fn second() -> Result<i32, std::io::Error> {
    let grid = parse_to_grid()?;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] == 'A' {
                if check_corners(&grid, row, col) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}
