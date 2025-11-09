use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, EnumIter)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

pub fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut count = 0;

    let mut grid: HashMap<Coord, char> = HashMap::new();

    for (x, line) in lines.map_while(Result::ok).enumerate() {
        for (y, char) in line.chars().enumerate() {
            grid.insert(
                Coord {
                    x: x as i32,
                    y: y as i32,
                },
                char,
            );
        }
    }

    // TODO could add additional logic to break early if position would mean it's not possible for word to be there

    for (k, v) in grid.iter() {
        if *v == 'X' {
            for dir in Direction::iter() {
                let start_coords = k.clone();
                let mut curr_coords = k.clone();
                let mut additional_chars = ['M', 'A', 'S'].iter().peekable();

                let mut found = false;

                while let Some(char) = additional_chars.next() {
                    curr_coords = match dir {
                        Direction::Right => Coord {
                            x: curr_coords.x,
                            y: curr_coords.y + 1,
                        },
                        Direction::Left => Coord {
                            x: curr_coords.x,
                            y: curr_coords.y - 1,
                        },
                        Direction::Up => Coord {
                            x: curr_coords.x - 1,
                            y: curr_coords.y,
                        },
                        Direction::Down => Coord {
                            x: curr_coords.x + 1,
                            y: curr_coords.y,
                        },
                        Direction::UpRight => Coord {
                            x: curr_coords.x - 1,
                            y: curr_coords.y + 1,
                        },
                        Direction::UpLeft => Coord {
                            x: curr_coords.x - 1,
                            y: curr_coords.y - 1,
                        },
                        Direction::DownRight => Coord {
                            x: curr_coords.x + 1,
                            y: curr_coords.y + 1,
                        },
                        Direction::DownLeft => Coord {
                            x: curr_coords.x + 1,
                            y: curr_coords.y - 1,
                        },
                    };

                    if curr_coords.x < 0 || curr_coords.y < 0 {
                        break;
                    }

                    // TODO sort out break out logic
                    if let Some(adjacent_char) = grid.get(&curr_coords) {
                        if *adjacent_char == *char {
                            if additional_chars.peek().is_none() {
                                count += 1;

                                let end_coords = &curr_coords;

                                // println!("Direction {dir:?}");
                                // println!("Start: {start_coords:?}");
                                // println!("End: {end_coords:?}");
                                // println!();
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return count;
}
