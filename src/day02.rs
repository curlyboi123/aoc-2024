use std::fs::File;
use std::io::{self};

#[derive(PartialEq, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

pub fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut safe_count: i32 = 0;

    for line in lines.map_while(Result::ok) {
        let mut is_safe = true;
        let mut direction: Option<Direction> = None;

        let report_levels: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let groups = report_levels.windows(2);

        for i in groups {
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

    return safe_count;
}
