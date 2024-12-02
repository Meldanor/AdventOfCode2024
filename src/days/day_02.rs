pub fn run(input: &Vec<String>) {
    println!("Hello, Day 02 here");
    let reports = parse_reports(input);

    // Part 1
    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();
    println!("(Part 1) {} reports are safe", safe_reports);

    // Part 2
    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe_with_tolerance(report))
        .count();
    println!("(Part 2) {} reports with tolerrance are safe", safe_reports);
}

// Part 1
fn is_report_safe(report: &Vec<u32>) -> bool {
    let limit = report.len() - 1;
    let must_increase = report[0] < report[1];
    let mut i = 0;
    while i < limit {
        let first = report[i];
        let second = report[i + 1];
        if must_increase && first > second || !must_increase && second > first {
            return false;
        }
        let difference = first.abs_diff(second);
        if difference < 1 || difference > 3 {
            return false;
        }

        i += 1;
    }
    return true;
}

// Part 2
fn is_report_safe_with_tolerance(report: &Vec<u32>) -> bool {
    // Try conventional safe check
    let res = is_report_safe(report);
    if res {
        return true;
    }
    // If one safe check failes remove one level by another and try it again with this one
    // Looks like brute force but I have no other idea
    for i in 0..report.len() {
        let mut report_missing_one_level = report.clone();
        report_missing_one_level.remove(i);
        if is_report_safe(&report_missing_one_level) {
            return true;
        }
    }
    return false;
}

fn parse_reports(input: &Vec<String>) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
