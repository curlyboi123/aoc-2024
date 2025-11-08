use regex::Regex;
use std::fs::File;
use std::io::{self};

pub fn part1(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();
    let mut total: i32 = 0;

    for line in lines.map_while(Result::ok) {
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
    return total;
}

pub fn part2(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let re = Regex::new(r"(mul\(\s*(\d+)\s*,\s*(\d+)\s*\))|(don't\(\))|(do\(\))").unwrap();
    let mut total: i32 = 0;

    let mut is_disabled = false;

    for line in lines.map_while(Result::ok) {
        let matches: Vec<_> = re.find_iter(&line).map(|m| m.as_str()).collect();
        let mul_re = Regex::new(r"^mul\(\s*(\d+)\s*,\s*(\d+)\s*\)$").unwrap();
        for m in matches {
            if m == "don't()" {
                is_disabled = true;
            } else if m == "do()" {
                is_disabled = false;
            } else if let Some(nums) = mul_re.captures(m) {
                if is_disabled {
                    continue;
                }
                let a: i32 = nums[1].parse().unwrap();
                let b: i32 = nums[2].parse().unwrap();
                let product = a * b;
                total += product;
            }
        }
    }
    return total;
}
