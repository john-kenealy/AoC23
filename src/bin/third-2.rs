use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;

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

    
    for line in grid {
        for char in line {
            if char.is_digit(10);
        }
    }


    // iterate through each line of input
}


fn adjacent(grid : &Vec<Vec<char>>, index: [usize; 2]) {

    println!("{:?}", grid);
}
