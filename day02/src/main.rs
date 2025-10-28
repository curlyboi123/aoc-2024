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

#[derive(PartialEq, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

fn main() {
    let mut safe_count: i32 = 0;

    if let Ok(lines) = read_lines("../inputs/02/short.txt") {
        for line in lines.map_while(Result::ok) {
            let mut is_safe = true;
            let mut direction: Option<Direction> = None;
            // println!("{:?}", line);
            let report_levels: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            println!("Line {line}");

            let groups_of_three = report_levels.windows(3);
            // for level in report_levels.iter() {
            //     println!("{level}");
            // }

            println!("groups of three: {:?}", groups_of_three);

            let initial_direction: Direction;

            for group in groups_of_three {
                println!("Group: {:?}", group);
            }
        }
    }
}
