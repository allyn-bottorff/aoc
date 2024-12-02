use std::collections::HashMap;

fn main() {
    // part 1 sample
    let mut v1_sample = vec![3, 4, 2, 1, 3, 3];
    let mut v2_sample = vec![4, 3, 5, 3, 9, 3];

    v1_sample.sort();
    v2_sample.sort();

    let tuples = v1_sample.iter().zip(v2_sample.iter());

    let total_diffs: i32 = tuples.map(|x| i32::abs(x.0 - x.1)).sum();
    println!("part 1 sample: {}", total_diffs);

    let content = std::fs::read_to_string("data.txt").unwrap();
    let lines = content.lines();

    // part 1 solution
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        v1.push(parts.next().unwrap().parse().unwrap());
        v2.push(parts.next().unwrap().parse().unwrap());
    }

    v1.sort();
    v2.sort();

    let tuples = v1.iter().zip(v2.iter());

    let total_diffs: i32 = tuples.map(|x| i32::abs(x.0 - x.1)).sum();
    println!("part 1 solution: {}", total_diffs);

    // part 2 sample
    let mut right: HashMap<i32, i32> = HashMap::new();
    for loc in v2_sample {
        if let Some(right_loc) = right.get_mut(&loc) {
            *right_loc += 1;
        } else {
            right.insert(loc, 1);
        }
    }

    let similarity_score: i32 = v1_sample
        .iter()
        .map(|x| x * right.get(x).unwrap_or(&0))
        .sum();

    println!("part 2 sample: {}", similarity_score);

    // part 2 solution

    let mut right: HashMap<i32, i32> = HashMap::new();
    for loc in v2 {
        if let Some(right_loc) = right.get_mut(&loc) {
            *right_loc += 1;
        } else {
            right.insert(loc, 1);
        }
    }
    let similarity_score: i32 = v1.iter().map(|x| x * right.get(x).unwrap_or(&0)).sum();
    println!("part 2 solution: {}", similarity_score);
}
