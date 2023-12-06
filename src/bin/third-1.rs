use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let doc = File::open("third-input").unwrap();
    let buf = io::BufReader::new(doc);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in buf.lines() {
        let mut inner_grid: Vec<char> = Vec::new();
        if let Ok(mut x) = line {
            while x.len() > 0 {
                inner_grid.push(x.pop().unwrap());
            }
            inner_grid.reverse();
            grid.push(inner_grid);
        }
    }

    let mut part_nums: Vec<usize> = vec![];

    // iterate through each line of input
    for i in 0..grid.len() {
        // iterate through each character
        for j in 0..grid[i].len() {
            // skip any non-digit, or any digit that follows a digit
            if (j > 0 && grid[i][j - 1].is_digit(10)) || !grid[i][j].is_digit(10) {
                continue;
            }

            let mut part_num = String::new();
            let mut counter = 0;

            // given a digit is found, grab all digits that immediately follow
            while j + counter < grid[i].len() && grid[i][j + counter].is_digit(10) {
                part_num.push(grid[i][j + counter]);
                counter += 1
            }

            if part_num.is_empty() {
                println!("got a live one, {}, [{},{}]", grid[i][j], i, j);
            }

            let mut adjacent_flag = false;

            for y in 0..=2 {
                if adjacent_flag {
                    break;
                }

                if (i == 0 && y == 0) || (i == grid.len() - 1 && y == 2) {
                    continue;
                }
                for x in 0..=counter+1 {
                    if (j == 0 && x == 0) || (j + x > grid[i].len()) {
                        continue;
                    }

                    let adjacent_char = grid[i + y - 1][j + x - 1];

                    if adjacent_char.is_digit(10) || adjacent_char == '.' {
                    } else {
                        part_nums.push(part_num.parse::<usize>().unwrap());
                        // part_nums.push(counter.try_into().unwrap());
                        adjacent_flag = true;
                        break;
                    }
                }
            }
        }
    }

    println!("{:?}", part_nums.iter().sum::<usize>());
}
