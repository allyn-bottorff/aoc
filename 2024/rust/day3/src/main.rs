use regex::Regex;

fn main() {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    let mut fields: Vec<&str> = Vec::new();

    for (_, [f1]) in re.captures_iter(test_input).map(|caps| caps.extract()) {
        fields.push(f1);
    }
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    for field in fields {
        let pair: Vec<&str> = field.split(',').collect();

        let p1 = pair[0].parse::<i32>().unwrap();
        let p2 = pair[1].parse::<i32>().unwrap();
        pairs.push((p1, p2));
    }

    let sum: i32 = pairs.iter().map(|p| p.0 * p.1).sum();

    println!("test data sum: {}", sum);

    let contents = std::fs::read_to_string("data.txt").unwrap();

    let mut fields: Vec<&str> = Vec::new();

    for (_, [f1]) in re.captures_iter(&contents).map(|caps| caps.extract()) {
        fields.push(f1);
    }
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    for field in fields {
        let pair: Vec<&str> = field.split(',').collect();

        let p1 = pair[0].parse::<i32>().unwrap();
        let p2 = pair[1].parse::<i32>().unwrap();
        pairs.push((p1, p2));
    }

    let sum: i32 = pairs.iter().map(|p| p.0 * p.1).sum();

    println!("input data sum: {}", sum);
}
