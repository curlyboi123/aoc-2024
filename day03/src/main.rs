use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {args:?}");

    let input_type = &args[1];
    match input_type {
        _ if input_type == "short" || input_type == "long" => (),
        _ => panic!("Invalid input type supplied. Allowed values "),
    }
    let input_path = format!("../inputs/03/{input_type}.txt");

    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    let mut total = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines.map_while(Result::ok) {
            println!("Lines: {line:?}");

            let mut matches = vec![];
            for (_, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
                matches.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));
            }

            for m in matches {
                println!("Match: {m:?}");
                let (a, b) = (m.0, m.1);
                let product = a * b;
                total += product;
            }
        }
    }
    println!("Total: {total}");
}
