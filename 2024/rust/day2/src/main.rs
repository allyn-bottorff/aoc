fn main() {
    let test_data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    // grab and parse the test data
    let reports: Vec<Vec<i32>> = test_data
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    for report in &reports {
        println!("{}", is_safe(report));
    }
    let safe_reports = reports
        .iter()
        .filter(|r| is_safe(r))
        .collect::<Vec<_>>()
        .len();
    println!("test safe levels: {}", safe_reports);

    let content = std::fs::read_to_string("data.txt").unwrap();

    let reports: Vec<Vec<i32>> = content
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let safe_reports = reports
        .iter()
        .filter(|r| is_safe(r))
        .collect::<Vec<_>>()
        .len();
    println!("safe levels: {}", safe_reports)
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|x| x[1] - x[0]).collect();

    let bad_diffs: Vec<&i32> = diffs
        .iter()
        .filter(|n| n.abs() > 3 || n.abs() < 1)
        .collect();

    if bad_diffs.len() > 0 {
        return false;
    }

    let negatives: Vec<&i32> = diffs.iter().filter(|n| n.is_negative()).collect();
    if negatives.len() == diffs.len() {
        return true;
    }

    let positives: Vec<&i32> = diffs.iter().filter(|n| n.is_positive()).collect();
    if positives.len() == diffs.len() {
        return true;
    }
    false
}
