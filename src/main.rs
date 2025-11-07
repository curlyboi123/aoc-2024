mod day01;

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
    input: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let day = args[1].clone();
        let input = args[2].clone();

        Ok(Config { day, input })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Make into seperate function for handling args
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let (day, input_type) = (config.day, config.input);
    let input_path = format!("../inputs/{day}/{input_type}.txt");
    //

    if let Ok(lines) = read_lines(input_path) {
        day01::result(lines);
    };
}
