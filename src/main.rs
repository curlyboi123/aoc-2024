mod day01;
mod day02;
mod day03;

use core::panic;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Config {
    day: String,
    part: String,
    input: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Add validation to all this
        let day = args[1].clone();
        let part = args[2].clone();
        let input = args[3].clone(); // Handle lowercase this

        Ok(Config { day, part, input })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Make into seperate function for handling args
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let (day, part, input_type) = (config.day, config.part, config.input);
    let input_path = format!("./inputs/{day}/{input_type}.txt");

    if let Ok(lines) = read_lines(input_path) {
        let result = match day {
            _ if day == "01" => match part {
                _ if part == "1" => day01::part1(lines),
                _ if part == "2" => day01::part2(lines),
                _ => panic!("Invalid part supplied"),
            },
            _ if day == "02" => match part {
                _ if part == "1" => day02::part1(lines),
                _ if part == "2" => day02::part1(lines),
                _ => panic!("Invalid part supplied"),
            },
            _ if day == "03" => match part {
                _ if part == "1" => day03::part1(lines),
                _ if part == "2" => day03::part2(lines),
                _ => panic!("Invalid part supplied"),
            },
            _ => {
                panic!("Invalid day supplied")
            }
        };
        println!("{result}");
    } else {
        panic!("Invalid input path supplied");
    };
}
