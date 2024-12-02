pub fn run(input: &Vec<String>) {
    println!("Hello, Day 02 here");
    let reports = parse_reports(input);

    // Part 1
    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();
    println!("(Part 1) {} reports are safe", safe_reports);
}

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

fn parse_reports(input: &Vec<String>) -> Vec<Vec<u32>> {
    let mut reports: Vec<Vec<u32>> = Vec::new();

    for line in input {
        reports.push(
            line.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        );
    }

    reports
}
