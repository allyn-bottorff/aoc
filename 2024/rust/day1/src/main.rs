fn main() {
    println!("Hello, world!");

    let mut v1 = vec![3, 4, 2, 1, 3, 3];
    let mut v2 = vec![4, 3, 5, 3, 9, 3];

    v1.sort();
    v2.sort();

    let tuples = v1.iter().zip(v2.iter());

    let total_diffs: i32 = tuples.map(|x| i32::abs(x.0 - x.1)).sum();
    println!("{}", total_diffs);
}
