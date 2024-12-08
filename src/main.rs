pub mod solutions;

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use solutions::{day1::Day1, day2::Day2};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <problem>", args[0]);
        return;
    }

    let path = format!("problems/{}", args[1]);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Could not find '{}'", path),
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    run(lines, &args[1]);
}

fn run(lines: Vec<String>, problem: &str) {
    match problem {
        "1-1" => Day1::new(lines).part_1(),
        "1-2" => Day1::new(lines).part_2(),
        "2-1" => Day2::new(lines).part_1(),
        "2-2" => Day2::new(lines).part_2(),
        _ => panic!("Could not find the solver."),
    };
}
