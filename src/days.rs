use std::fs::read_to_string;

mod day_01;

pub fn run(day: u32) {
    println!("Running AdventOfCode Day {:02}...", day);
    let input = read_input(day);
    match day {
        1 => day_01::run(&input),
        _ => eprintln!("Unsupported day {}", day),
    }
}

fn read_input(day: u32) -> Vec<String> {
    let path = format!("inputs/day_{:02}", day);

    read_to_string(path)
        .expect("Can't open input file")
        .lines()
        .map(String::from)
        .collect()
}