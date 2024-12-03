use regex::Regex;

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
    let multiplications = parse_input_part_one(input);
    let mut sum: u64 = 0;
    for multiplication in multiplications {
        sum += multiplication.calculate();
    }
    println!("(Part 1) The sum is {}", sum);

    let multiplications = parse_input_part_two(input);
    let mut sum: u64 = 0;
    for multiplication in multiplications {
        sum += multiplication.calculate();
    }
    println!("(Part 2) The sum is {}", sum);
}

const MULTIPLICATION_PATTERN_PART_ONE: &str = r"mul\((\d+),(\d+)\)";
fn parse_input_part_one(input: &Vec<String>) -> Vec<Multiplication> {
    let flat_input_string = input.join("");
    let mut result: Vec<Multiplication> = Vec::new();

    let regex = Regex::new(MULTIPLICATION_PATTERN_PART_ONE).unwrap();

    for (_, [x, y]) in regex.captures_iter(&flat_input_string).map(|c| c.extract()) {
        match (x.parse::<u64>(), y.parse::<u64>()) {
            (Ok(x), Ok(y)) => result.push(Multiplication { x, y }),
            _ => eprintln!("Can't parse x or y"),
        }
    }

    return result;
}

// Idea: Match don't, do and mult operations and find out in the group if we need to ignore further operations or not
const MULTIPLICATION_PATTERN_PART_TWO: &str = r"(don't\(\))|(do\(\))|(?:(mul)\((\d+),(\d+)\))";
fn parse_input_part_two(input: &Vec<String>) -> Vec<Multiplication> {
    let flat_input_string = input.join("");
    let mut result: Vec<Multiplication> = Vec::new();

    let regex = Regex::new(MULTIPLICATION_PATTERN_PART_TWO).unwrap();
    let mut parse_instruction: bool = true;
    // Hint: We cannot use the Capture::extract function because the amount of groups changes
    for capture in regex.captures_iter(&flat_input_string) {
        let dont_group = capture.get(1);
        let do_group = capture.get(2);
        let mul_group = capture.get(3);
        let mul_x_group = capture.get(4);
        let mul_y_group = capture.get(5);
        match (dont_group, do_group, mul_group, mul_x_group, mul_y_group) {
            (Some(_dont), _, _, _, _) => parse_instruction = false,
            (_, Some(_do), _, _, _) => parse_instruction = true,
            (_, _, Some(_mult), Some(x), Some(y)) => {
                if !parse_instruction {
                    continue;
                }
                result.push(Multiplication {
                    x: x.as_str().parse::<u64>().unwrap(),
                    y: y.as_str().parse::<u64>().unwrap(),
                });
            }
            _ => panic!(),
        }
    }

    return result;
}
