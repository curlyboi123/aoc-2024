use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::thread::current;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn left_coord(&self) -> Self {
        let Self { x, y } = self;
        let y = y + 1;
        return Self { x: *x, y };
    }
}

pub fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut count = 0;

    let mut grid: HashMap<Coord, char> = HashMap::new();

    for (x, line) in lines.map_while(Result::ok).enumerate() {
        // println!("Line: {y}");
        // println!("{idx}");
        for (y, char) in line.chars().enumerate() {
            // println!("Chars: {char}");

            grid.insert(Coord { x, y }, char);
        }
    }

    // TODO could add additional logic to break early if position would mean it's not possible for word to be there

    for (k, v) in grid.iter() {
        if *v == 'X' {
            // Check characters to the right
            let start_coords = k;
            let mut additional_chars = ['M', 'A', 'S'].iter().peekable();
            let mut curr_coords = k.clone();
            while let Some(char) = additional_chars.next() {
                curr_coords = Coord {
                    x: curr_coords.x,
                    y: curr_coords.y + 1,
                };
                if let Some(adjacent_char) = grid.get(&curr_coords) {
                    if *adjacent_char == *char {
                        if additional_chars.peek().is_none() {
                            let end_coords = &curr_coords;
                            println!(
                                "Word 'XMAS' found between coords: {start_coords:?} and {end_coords:?}"
                            );
                            count += 1;
                        }
                    }
                } else {
                    break;
                }
            }

            // Check vertical

            // Check diagonal
        }
    }

    // println!("Grid: {grid:?}");

    return count;
}
