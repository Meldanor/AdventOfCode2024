use std::{env, process::exit};

mod days;
mod map_2d;
mod point_2d;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Missing day selector as parameter!");
        exit(-1);
    }
    let day_selection: u32 = args[1].parse().expect("Day selection must be an integer");
    days::run(day_selection);
}
