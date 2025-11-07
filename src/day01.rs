use std::collections::HashMap;
use std::fs::File;
use std::io;

fn count_value_instances(vec: &Vec<i32>, val: i32) -> i32 {
    let count = vec.iter().filter(|&n| *n == val).count();
    return count as i32;
}

fn get_similarity_score(val: i32, count: i32) -> i32 {
    val * count
}

pub fn result(lines: io::Lines<io::BufReader<File>>) {
    // Read into 2 seperate lists
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let foo: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        a.push(foo[0]);
        b.push(foo[1]);
    }

    // Part 1
    a.sort();
    b.sort();
    let diff: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).collect();
    let diff_sum: i32 = diff.iter().sum();
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
