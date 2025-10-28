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

    if let Ok(lines) = read_lines("../inputs/02/long.txt") {
        for line in lines.map_while(Result::ok) {
            let mut is_safe = true;
            let mut direction: Option<Direction> = None;
            // println!("{:?}", line);
            let report_levels: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let foo = report_levels.windows(2);
            for i in foo {
                let diff = (i[1] - i[0]).abs();
                if diff < 1 || diff > 3 {
                    is_safe = false;
                    break;
                }

                match direction {
                    Some(Direction::Increasing) => {
                        if i[1] < i[0] {
                            is_safe = false;
                            break;
                        }
                    }
                    Some(Direction::Decreasing) => {
                        if i[1] > i[0] {
                            is_safe = false;
                            break;
                        }
                    }
                    None => {
                        if i[1] > i[0] {
                            direction = Some(Direction::Increasing)
                        } else {
                            direction = Some(Direction::Decreasing)
                        }
                    }
                }
            }
            if is_safe {
                safe_count += 1;
            }
        }
    }

    println!("Safe count: {}", safe_count);
}
