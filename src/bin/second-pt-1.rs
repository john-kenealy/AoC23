use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let doc: String = read_to_string("second-input").unwrap();

    let game_reg = Regex::new(r"Game (.*):").unwrap();
    let red_reg = Regex::new(r"(?:(\d{1,2}) red)+").unwrap();
    let blue_reg = Regex::new(r"(?:(\d{1,2}) blue)+").unwrap();
    let green_reg = Regex::new(r"(?:(\d{1,2}) green)+").unwrap();

    const RMAX: u8 = 12;
    const BMAX: u8 = 14;
    const GMAX: u8 = 13;

    // let mut parsed_game = vec![];

    let mut feasible_games : Vec<u32> = vec![];

    for line in doc.lines() {
        let mut cube_counts = vec![];
        

        //check for red impossibles
        for (_, [cubes]) in red_reg.captures_iter(line).map(|c| c.extract()) {
            cube_counts.push(cubes.parse::<u8>().unwrap());
        };
        println!("Line {} red cubes {:?}", line, cube_counts);
        if cube_counts.iter().max().unwrap() > &RMAX {
            continue;
        }
        cube_counts.clear();

        //check for blue impossibles
        for (_, [cubes]) in blue_reg.captures_iter(line).map(|c| c.extract()) {
            cube_counts.push(cubes.parse::<u8>().unwrap());
        };
        println!("Line {} blue cubes {:?}", line, cube_counts);
        if cube_counts.iter().max().unwrap() > &BMAX {
            continue;
        }
        cube_counts.clear();

        //check for green impossibles
        for (_, [cubes]) in green_reg.captures_iter(line).map(|c| c.extract()) {
            cube_counts.push(cubes.parse::<u8>().unwrap());
        };
        println!("Line {} green cubes {:?}", line, cube_counts);
        if cube_counts.iter().max().unwrap() > &GMAX {
            continue;
        }



        let game_c: u32 = game_reg.captures(line).unwrap().extract::<1>().1[0].parse::<u32>().unwrap();

        println!("{:?}", game_c);

        feasible_games.push(game_c);
    }

    println!("Sum of feasible games {}", feasible_games.iter().sum::<u32>());
}
