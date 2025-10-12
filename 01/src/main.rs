use std::collections::HashMap;
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

fn sum_vector(a: &Vec<i32>) -> i32 {
    a.iter().sum()
}

fn count_value_instances(vec: &Vec<i32>, val: i32) -> i32 {
    let count = vec.iter().filter(|&n| *n == val).count();
    return count as i32;
}

fn get_similarity_score(val: i32, count: i32) -> i32 {
    val * count
}

fn main() {
    // Read into 2 seperate lists
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./inputs/short.txt") {
        for line in lines.map_while(Result::ok) {
            let foo: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            a.push(foo[0]);
            b.push(foo[1]);
        }
    }

    // Part 1
    a.sort();
    b.sort();
    let diff: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).collect();
    let diff_sum: i32 = sum_vector(&diff);
    println!("Diffs Sum: {:?}", diff_sum);

    // Part 2
    let mut counts: HashMap<i32, i32> = HashMap::new();

    let mut total: i32 = 0;
    for val in a.iter() {
        let result = counts.get(val);
        let count = match result {
            None => count_value_instances(&b, *val),
            Some(i) => *i,
        };
        counts.insert(*val, count);
        // let sim_score: i32 = val * count;
        total += get_similarity_score(*val, count);
    }
    println!("Simalirty Score: {:?}", total);
}
