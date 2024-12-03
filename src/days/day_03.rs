use regex::Regex;

const MULTIPLICATION_PATTERN: &str = r"mul\((\d+),(\d+)\)";

trait MathOperation {
    fn calculate(&self) -> u64;
}

#[derive(Debug)]
struct Multiplication {
    x: u64,
    y: u64,
}

impl MathOperation for Multiplication {
    fn calculate(&self) -> u64 {
        self.x * self.y
    }
}

pub fn run(input: &Vec<String>) {
    let multiplications = parse_input(input);
    let mut sum: u64 = 0;
    for multiplication in multiplications {
        sum += multiplication.calculate();
    }
    println!("(Part 1) The sum is {}", sum);
}

fn parse_input(input: &Vec<String>) -> Vec<Multiplication> {
    let flat_input_string = input.join("");
    let mut result: Vec<Multiplication> = Vec::new();

    let regex = Regex::new(MULTIPLICATION_PATTERN).unwrap();

    for (_, [x, y]) in regex.captures_iter(&flat_input_string).map(|c| c.extract()) {
        match (x.parse::<u64>(), y.parse::<u64>()) {
            (Ok(x), Ok(y)) => result.push(Multiplication { x, y }),
            _ => eprintln!("Can't parse x or y"),
        }
    }

    return result;
}
