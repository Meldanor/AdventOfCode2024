use std::fs::read_to_string;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

pub fn run(day: u32) {
    println!("Running AdventOfCode Day {:02}...", day);
    let input = read_input(day);
    match day {
        1 => day_01::run(&input),
        2 => day_02::run(&input),
        3 => day_03::run(&input),
        4 => day_04::run(&input),
        5 => day_05::run(&input),
        6 => day_06::run(&input),
        7 => day_07::run(&input),
        8 => day_08::run(&input),
        9 => day_09::run(&input),
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
