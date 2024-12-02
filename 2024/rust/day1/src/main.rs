fn main() {
    let mut v1 = vec![3, 4, 2, 1, 3, 3];
    let mut v2 = vec![4, 3, 5, 3, 9, 3];

    v1.sort();
    v2.sort();

    let tuples = v1.iter().zip(v2.iter());

    let total_diffs: i32 = tuples.map(|x| i32::abs(x.0 - x.1)).sum();
    println!("sample data: {}", total_diffs);

    let content = std::fs::read_to_string("data.txt").unwrap();
    let lines = content.lines();

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
    println!("test data: {}", total_diffs);
}
